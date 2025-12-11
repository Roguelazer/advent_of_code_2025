use itertools::Itertools;
use petgraph::graph::DiGraph;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use tap::Pipe;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct DeviceId(u32);

#[derive(Debug)]
struct Problem {
    devices: BTreeMap<String, DeviceId>,
    connections: BTreeMap<DeviceId, BTreeSet<DeviceId>>,
    graph: DiGraph<String, ()>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let stdin = std::io::stdin();
        let stdin_lock = stdin.lock();
        let s = std::io::read_to_string(stdin_lock)?;
        let mut devices = BTreeMap::new();
        let mut connections = BTreeMap::new();
        let mut next_id = 1;
        for line in s.lines() {
            let Some((this_device, rest)) = line.split_once(": ") else {
                anyhow::bail!("invalid line {:?}", line)
            };
            let device_id = *devices.entry(this_device.to_owned()).or_insert_with(|| {
                let id = DeviceId(next_id);
                next_id += 1;
                id
            });
            let device_connections: BTreeSet<DeviceId> = rest
                .split_whitespace()
                .map(|other| {
                    *devices.entry(other.to_owned()).or_insert_with(|| {
                        let id = DeviceId(next_id);
                        next_id += 1;
                        id
                    })
                })
                .collect();
            connections.insert(device_id, device_connections);
        }
        let mut graph = DiGraph::<String, ()>::new();
        for (name, _) in devices.iter().sorted_by_key(|(_, i)| i.0) {
            graph.add_node(name.clone());
        }
        graph.extend_with_edges(
            connections
                .iter()
                .flat_map(|(source, dests)| dests.iter().map(|dest| (source.0, dest.0))),
        );
        if petgraph::algo::is_cyclic_directed(&graph) {
            anyhow::bail!("there is a cycle");
        }
        Ok(Self {
            devices,
            connections,
            graph,
        })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        let Some(start) = self.devices.get("you").copied() else {
            anyhow::bail!("could not find 'you'");
        };
        let Some(finish) = self.devices.get("out").copied() else {
            anyhow::bail!("could not find 'out'");
        };
        self.count_paths_between(&self.graph, start, finish)
            .pipe(Ok)
    }

    fn part2(&self) -> anyhow::Result<usize> {
        let Some(start) = self.devices.get("svr").copied() else {
            anyhow::bail!("could not find 'svr'");
        };
        let Some(finish) = self.devices.get("out").copied() else {
            anyhow::bail!("could not find 'out'");
        };
        let Some(fft) = self.devices.get("fft").copied() else {
            anyhow::bail!("could not find 'fft'");
        };
        let Some(dac) = self.devices.get("dac").copied() else {
            anyhow::bail!("could not find 'dac'");
        };

        // good news: there are no paths from dac to fft, which means that all paths must
        // have the form of `svr -> fft` followed by `fft -> dac`, followed by `dac -> out`.
        //
        // furthermore, there are no cycles, so any path that ever traverses a node "below" FFT
        // cannot get back to it.

        let to_fft = self.count_paths_between(&self.graph, start, fft);
        tracing::debug!("found {} paths between start and fft", to_fft);
        let to_dac = self.count_paths_between(&self.graph, fft, dac);
        tracing::debug!("found {} paths between fft and dac", to_dac);
        let to_end = self.count_paths_between(&self.graph, dac, finish);
        tracing::debug!("found {} paths between dac and finish", to_end);

        Ok(to_fft * to_dac * to_end)
    }

    fn count_paths_between(
        &self,
        graph: &DiGraph<String, ()>,
        start: DeviceId,
        finish: DeviceId,
    ) -> usize {
        let mut memo = BTreeMap::new();
        memo.insert(finish, 1);

        let mut sorted_nodes = petgraph::algo::toposort(&graph, None)
            .unwrap()
            .into_iter()
            .map(|n| DeviceId(n.index() as u32))
            .collect::<Vec<_>>();
        sorted_nodes.reverse();
        let start_idx = sorted_nodes.iter().position(|n| *n == finish).unwrap() + 1;
        let finish_idx = sorted_nodes.iter().position(|n| *n == start).unwrap();
        assert!(start_idx < finish_idx);

        for node in &sorted_nodes[start_idx..=finish_idx] {
            if let Some(neighbors) = self.connections.get(&node) {
                memo.insert(
                    *node,
                    neighbors
                        .iter()
                        .filter_map(|n| memo.get(n).copied())
                        .sum::<usize>(),
                );
            }
        }

        memo.get(&start).copied().unwrap_or(0)
    }

    #[allow(unused)]
    fn dump_to_dot(&self, fname: &str) -> anyhow::Result<()> {
        let mut f = std::fs::File::create(fname)?;
        write!(f, "digraph G {{\n")?;
        for (name, index) in self.devices.iter() {
            write!(f, "  node{} [label=\"{}\"]\n", index.0, name)?;
        }
        for (source, dests) in self.connections.iter() {
            for dest in dests.iter() {
                write!(f, "  node{} -> node{}\n", source.0, dest.0)?;
            }
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let problem = Problem::read().unwrap();
    println!("part 1: {}", problem.part1().unwrap());
    println!("part 2: {}", problem.part2().unwrap());
}
