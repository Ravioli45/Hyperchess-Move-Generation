use core::panic;
use std::{fs::File, io::Read};
use hmg::{Position, MoveList};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct TestPosition{
    fen: String,
    nodes: Vec<usize>,
    depth: usize,
}

fn load_test_suite() -> Vec<TestPosition>{

    let mut test_suite_file = File::open("tests/test-suite.json").unwrap();

    let mut test_suite_string: String = String::new();

    let _ = test_suite_file.read_to_string(&mut test_suite_string);

    serde_json::from_str(&test_suite_string).unwrap()
}

fn run_test_position(position: &mut Position, depth: usize, max_depth: usize, move_counter: &mut Vec<usize>){

    move_counter[depth] += 1;
    //println!("{position}");
    
    if depth >= max_depth{
        return;
    }

    let moves: MoveList = position.generate_moves();

    for m in moves{

        //println!("{m:?}");

        if !position.is_move_legal(m){
            //println!("h");
            continue;
        }
        //println!("h");
        
        //println!("{:?}", m);
        position.make_move(m);

        run_test_position(position, depth+1, max_depth, move_counter);
        
        position.unmake_move(m);
    }
}

#[test]
fn run_test_suite(){

    println!("Loading test suite...");
    let test_suite: Vec<TestPosition> = load_test_suite();
    println!("Finished loading test suite");

    //println!("{test_suite:?}");

    let mut passed: usize = 0;
    let total_tests: usize = test_suite.len();
    println!("Running test suite...\n");

    for (i, test) in test_suite.into_iter().enumerate(){

         /*
        if i < 90{
            continue;
        }
        // */

        println!("Running test {}/{}: {}", i+1, total_tests, test.fen);

        let mut move_counter: Vec<usize> = vec![0; test.depth+1];
    
        let mut test_position = Position::from_FEN(&test.fen).unwrap();

        run_test_position(&mut test_position, 0, test.depth, &mut move_counter);
        
        println!("Expected: {:?}", test.nodes);
        println!("Observed: {:?}", move_counter);

        //assert_eq!(test.nodes, move_counter);
        //break;
        if test.nodes == move_counter{
            println!("Passed");
            passed += 1;
        }
        else{
            println!("Failed");
        }

        //break;
        println!()
    }
    println!();

    println!("{passed}/{total_tests} test cases passed");

    if passed < total_tests{
        panic!();
    }
}