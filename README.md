**Jikan (時間)** is a Rust library to batch-execute, time, and verify solutions to Advent of Code puzzles.

## Code Example
```rs
struct Manifests;
impl ManifestProvider for Manifests {
    fn get_manifest(day: Day) -> Result<DayManifest, Box<dyn Error>> {
        let path = format!("data/{}/day_{:02}.yaml", day.year, day.day);
        let file = File::open(&path)?;
        let manifest = serde_yml::from_reader(file)?;
        Ok(manifest)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = jikan::ExecutionOptions::from_args();

    let solvers: HashMap<Puzzle, Solver> = [
        (Puzzle { year: 2023, day: 1, part: 1 }, solvers_2023::day_01::solve_part_1 as Solver)
    ].into_iter().collect();

    jikan::execute::<Manifests, _, _>(options, &solvers);

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