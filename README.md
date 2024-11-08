**Jikan (時間)** is a Rust library to batch-execute, time, and verify solutions to Advent of Code puzzles.

## Code Example
```rs
fn main() -> Result<(), Box<dyn Error>> {
    let options = jikan::ExecutionOptions::from_args();

    let file = File::open("data.yaml")?;
    let data = serde_yml::from_reader(file)?;

    let solvers: HashMap<Puzzle, Solver> = [
        (Puzzle { year: 2023, day: 1, part: 1 }, solvers_2023::day_01::solve_part_1 as Solver)
    ].into_iter().collect();

    let manifest = jikan::Manifest { solvers, data };
    jikan::execute(options, &manifest);

    Ok(())
}
```

The inputs, and solutions can be parsed using any serde compatible parser.
Here is an example YAML structure
```yaml
puzzles: 
  2023-01:
    input: |-
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    parts:
    - solution: 142
```