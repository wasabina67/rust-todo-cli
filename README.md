# rust-todo-cli
Rust Todo cli

## Init

```bash
cargo init
```

## Add dependencies

```bash
cargo add serde --features derive
cargo add clap --features derive
cargo add serde_json
cargo add dirs
```

## Usage

### Add

```bash
cargo run -- add "task-abc"
```

### List

```bash
cargo run -- list
```

### Complete

```bash
cargo run -- complete 1
```

### Remove

```bash
cargo run -- remove 1
```

### Help

```bash
cargo run -- --help
```

## Build

```bash
cargo build
```

```bash
cargo build --release
```

## Run

```bash
cargo run -- list
```

```bash
./target/release/todo-cli list
```
