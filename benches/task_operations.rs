use criterion::{Criterion, black_box, criterion_group, criterion_main};
use taskclaw::task::TaskList;

fn benchmark_add_tasks(c: &mut Criterion) {
    c.bench_function("add_task", |b| {
        b.iter(|| {
            let mut task_list = TaskList::new();
            for i in 0..1000 {
                task_list.add_task(black_box(format!("Task {}", i))).unwrap();
            }
        })
    });
}

fn benchmark_complete_tasks(c: &mut Criterion) {
    let mut task_list = TaskList::new();
    for i in 0..1000 {
        task_list.add_task(format!("Task {}", i)).unwrap();
    }

    c.bench_function("complete_task", |b| {
        let mut task_list = task_list.clone();
        let mut counter = 0;
        b.iter(|| {
            task_list.complete_task(black_box(counter % 1000));
            counter += 1;
        })
    });
}

fn benchmark_remove_tasks(c: &mut Criterion) {
    c.bench_function("remove_task", |b| {
        b.iter(|| {
            let mut task_list = TaskList::new();
            for i in 0..100 {
                task_list.add_task(format!("Task {}", i)).unwrap();
            }
            for i in 0..50 {
                task_list.remove_task(black_box(i));
            }
        })
    });
}

fn benchmark_large_tasklist_operations(c: &mut Criterion) {
    c.bench_function("large_tasklist_search", |b| {
        let mut task_list = TaskList::new();
        for i in 0..10000 {
            task_list.add_task(format!("Task {}", i)).unwrap();
        }

        b.iter(|| {
            task_list.get_task(black_box(5000));
        })
    });
}

criterion_group!(
    benches,
    benchmark_add_tasks,
    benchmark_complete_tasks,
    benchmark_remove_tasks,
    benchmark_large_tasklist_operations
);
criterion_main!(benches);
