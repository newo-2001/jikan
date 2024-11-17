**Jikan (時間)** is a Rust library to batch-execute, time, and verify solutions to Advent of Code puzzles.

## Code Example
```rs
fn main() -> Result<(), Box<dyn Error>> {
    let options = jikan::ExecutionOptions::from_args();

    let puzzles: HashMap<Day, DayManifest> = jikan::locate_manifests(Path::new("data"), options.scope)?
        .into_iter()
        .map(|(day, path)| {
            let file = File::open(path)?;
            let manifest = serde_yml::from_reader(file)?;
            Ok((day, manifest))
        })
        .collect::<anyhow::Result<_>>()?;


    let solvers: HashMap<Puzzle, Solver> = [
        (Puzzle { year: 2023, day: 1, part: 1 }, solvers_2023::day_01::solve_part_1 as Solver)
    ].into_iter().collect();

    let manifest = jikan::Manifest { solvers, data };
    jikan::execute(options, &manifest);

    Ok(())
}
```

The inputs, and solutions can be parsed using any serde compatible deserializer.
Here is an example YAML structure
```yaml
input: |-
  1abc2
  pqr3stu8vwx
  a1b2c3d4e5f
  treb7uchet
parts:
  - solution: 142
```