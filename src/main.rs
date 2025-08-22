fn main() {
    println!("Hello, world!");
}

/*
1. Branch feature/core
git checkout -b feature/core
git add src/core/*.rs
git commit -m "feat(core): initial commit for core module"

2. Branch feature/frontend
git checkout main
git checkout -b feature/frontend
git add src/frontend/*.rs
git commit -m "feat(frontend): initial commit for frontend module"

3. Branch feature/executor
git checkout main
git checkout -b feature/executor
git add src/executor/*.rs
git commit -m "feat(executor): initial commit for executor module"

4. Branch feature/storage
git checkout main
git checkout -b feature/storage
git add src/storage/*.rs
git commit -m "feat(storage): initial commit for storage module"

5. Branch feature/interface
git checkout main
git checkout -b feature/interface
git add src/interface.rs src/lib.rs
git commit -m "feat(interface): add interface module and update lib API"

6. Branch feature/lib
git checkout main
git checkout -b feature/lib
git add src/lib.rs
git commit -m "feat(lib): initial commit for library API"

7. Branch feature/repl
git checkout main
git checkout -b feature/repl
git add src/main.rs
git commit -m "feat(repl): add main REPL entry point using lib"

 */