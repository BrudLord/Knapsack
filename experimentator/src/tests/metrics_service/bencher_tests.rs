use super::*;
use crate::metrics_service::{bencher::{self, Bencher}, models::measurement::Measurement};
use knapsack_library::{
    algorithms_service::AlgorithmsService,
    models::{item::Item, knapsack::Knapsack, knapsack_solver::KnapsackSolver},
};
use std::{collections::HashMap, env, fs, hash::Hash, path::Path};

/// Helper function to create test knapsacks
fn create_test_knapsacks() -> Vec<Knapsack> {
    vec![
        Knapsack::new(10, vec![Item::new(5, 10), Item::new(4, 8)]),
        Knapsack::new(8, vec![Item::new(3, 6), Item::new(4, 7)]),
    ]
}

fn create_test_solvers() -> Vec<Box<dyn KnapsackSolver>> {
    let algorithms = AlgorithmsService::get_all_algorithms();
    algorithms
        .into_iter()
        .filter(|algorithm| algorithm.get_name() == "Dynamic" || algorithm.get_name() == "Greedy")
        .collect()
}

#[test]
fn test_bencher_creation() {
    let bencher = Bencher::new(None, false);
    assert!(bencher.is_ok());
}

fn test_bencher_with_reporter() {
    let os_string = "linux";

    // Create bencher with the temporary file
    let bencher = Bencher::new(Some(os_string), true).unwrap();
    let knapsacks = create_test_knapsacks();
    let solvers = create_test_solvers();

    // Conduct experiment
    bencher.conduct_experiment(&solvers, &knapsacks, os_string);

    // Read the file content
    let mut start_dir = env::current_dir().expect("Failed to get current working directory");
    start_dir = start_dir.parent().unwrap().to_path_buf();
    let experiment_results_dir = start_dir
                    .join("docs".to_string())
                    .join("experiments".to_string())
                    .join(os_string);
    println!("{}", experiment_results_dir.display());

    let content = fs::read_to_string(experiment_results_dir.join("experiment.md")).unwrap();

    // Verify the output contains expected elements
    assert!(content.contains("Algorithm"));
    assert!(content.contains("Success Rate"));
    assert!(content.contains("Execution Time"));
    //assert!(content.contains("Dynamic"));
    assert!(content.contains("Greedy"));

}


#[test]
fn test_bench_group_empty_knapsacks() {
    let bencher = Bencher::new(None, false).unwrap();
    let solvers = create_test_solvers();
    let empty_knapsacks: Vec<Knapsack> = vec![];

    // Should not panic with empty knapsacks
    bencher.bench_group(&solvers, &empty_knapsacks, None, None, None, None);
}

#[test]
fn test_cleanup_after_experiment() {
    let bencher = Bencher::new(None, false).unwrap();
    let knapsacks = create_test_knapsacks();
    let solvers = create_test_solvers();
    let os_string = "linux";


    bencher.conduct_experiment(&solvers, &knapsacks, os_string);

    // Verify that criterion directory is cleaned up
    assert!(!Path::new("target/criterion").exists());
}
