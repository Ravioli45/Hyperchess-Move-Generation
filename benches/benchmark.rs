use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use hmg::{Position, MoveList};

fn run_test_position(position: &mut Position, depth: usize, max_depth: usize){

    if depth >= max_depth{
        return;
    }

    let moves: MoveList = position.generate_moves();

    for m in moves{
        //println!("{:?}", m);
        position.make_move(m);

        run_test_position(position, depth+1, max_depth);
        
        position.unmake_move(m);
    }
}

fn from_start_position(){

    let mut p = Position::from_start_position();

    run_test_position(&mut p, 0, 4);
}

fn benchmark(c: &mut Criterion){

    c.bench_function("from start position", |b| b.iter(||
        black_box(from_start_position())
    ));

}

criterion_group!{name = hmg_bench; 
    config = Criterion::default().sample_size(10);
    targets = benchmark
}
criterion_main!(hmg_bench);
