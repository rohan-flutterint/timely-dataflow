use timely::dataflow::{InputHandle, ProbeHandle};
use timely::dataflow::operators::{Inspect, Probe};
use timely::WorkerConfig;

fn main() {

    // create a naked single-threaded worker.
    let allocator = timely::communication::allocator::Thread::default();
    let mut worker = timely::worker::Worker::new(WorkerConfig::default(), allocator, None);

    // create input and probe handles.
    let mut input = InputHandle::new();
    let probe = ProbeHandle::new();

    // directly build a dataflow.
    worker.dataflow(|scope| {
        input
            .to_stream(scope)
            .inspect(|x| println!("{:?}", x))
            .probe_with(&probe);
    });

    // manage inputs.
    for i in 0 .. 10 {
        input.send(i);
        input.advance_to(i);
        while probe.less_than(input.time()) {
            worker.step();
        }
    }
}
