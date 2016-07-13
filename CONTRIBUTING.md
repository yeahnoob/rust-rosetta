# Contributing #

* The code you contribute *is* public domain.
* Don't be afraid of comments: the code is going to be written once, read hundreds of times, and maintained well past when you submit it.
* Keep your code as simple as possible.
* Please avoid compiler warnings.
* Write tests.

Not sure how to help?

* Implement a solution for a Rosetta Code task (see below).
* Improve an existing solution (increase readability, performance, etc.).
* Add tests.
* Copy solutions from this repository to the [wiki].

# Implementing a Solution #

## TL;DR ##
* Pick a task to implement (see [rust-rosetta coverage](https://euclio.github.io/rosetta-coverage) for unimplemented tasks).
* Add an entry for your task in `Cargo.toml`.
* Add a link to the Rosetta code wiki page for your task at the top of your
  file.
* Implement your task.
* Add tests.

## Full Process ##
#### Initial setup ####

* Fork this repo on GitHub ([help here!](https://help.github.com/articles/fork-a-repo/)).
* Clone your fork onto your machine.

#### Every other time ####
* Make sure you are on the `master` branch.

  ```sh
  $ git checkout master
  ```

* Update your fork with upstream ([details](https://help.github.com/articles/syncing-a-fork/)).
* Find a task you want to implement on [Rosetta Code](https://rosettacode.org). Check if the task you want to implement already exists in the repository (see [`Cargo.toml`](Cargo.toml)) or the [wiki].
* Create and switch to a branch with a reasonably unique name:

  ```sh
  $ git checkout -b hoverbear-fizzbuzz
  ```

* Create a file in the repository that matches the name of the task.

    For example, `src/fizzbuzz.rs` for [FizzBuzz](http://rosettacode.org/wiki/FizzBuzz) or `src/hello_world/text.rs` for [Hello world/Text](http://rosettacode.org/wiki/Hello_world/Text).

    Any resources needed by the task should be added to the [`resources`](./resources) folder.

* Add your file to [`Cargo.toml`](Cargo.toml) (it's alphabetical!): If you'd like, you're welcome to add your contact details, name, or other information first to the `authors` array. Then add the entry in *lexicographical* ordering into [`Cargo.toml`](Cargo.toml) like this:

  ```toml
  [[bin]]
  # http://rosettacode.org/wiki/FizzBuzz
  name = "fizzbuzz"
  path = "src/fizzbuzz.rs"
  ```

* Include a link to the Rosetta Code Problem you are solving at the top of your file like this:

  ```rust
  // http://rosettacode.org/wiki/FizzBuzz
  ```

* Implement your task! You should add a `main()` function that demonstrates the solution. If your task is testable, please add tests to the bottom of the file. See below for more in-depth testing guidelines.

* Your code should build without warnings on latest nightly provided by [`rustup.sh`](https://www.rust-lang.org/downloads.html).
* Please, use [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) on your code.

    You can install the tool with `cargo install rustfmt`. See `rustfmt --help` for more information. If you're having trouble getting `rustfmt` to work, try to keep your contributions adherent to the official style guide which you can see at [this location](http://doc.rust-lang.org/nightly/style/). Note the style guide is still a work-in-progress.

* Stage your changes for commit, adding new and modified files to it:

  ```sh
  $ git add src/fizzbuzz.rs
  $ git add Cargo.toml
  ```
* Check `git status` to make sure you don't mangle anything else.
* Commit your changes.

  ```sh
  git commit -a -m "Implement blah blah blah"
  ```
* Push your branch.

  ```sh
  git push -u origin hoverbear-fizzbuzz
  ```

* Submit a [pull request](https://help.github.com/articles/creating-a-pull-request/).

#### Code Review ####

Once you've submitted your pull request, a collaborator on the project will review your code in preparation for merging. If you have to make changes, you can just push additional commits to your branch and they will be reflected in the PR.

##### Gotchas #####

Sometimes, while working on a PR, your branch will get out-of-date with the `master` branch. In this case, you can update `master` and `rebase` your branch onto the new `master`. If you want more information, please check out this [helpful guide](https://github.com/edx/edx-platform/wiki/How-to-Rebase-a-Pull-Request).

If your commit history becomes messy, you can use `git rebase -i` to squash multiple commits into one commit. You can then use `git push -f origin` to update your pull request. Make sure you only `git push -f` to your branch, not `master`!

#### After your PR is accepted ####

* Copy your solution to the wiki.
* Delete the branch

If this is unclear or you're having difficulties, just open an issue, we'd love to help.

#### Extra Credit ####

In addition to preventing warnings, you can try running [`cargo
clippy`](https://github.com/Manishearth/rust-clippy) on your code. `clippy`
provides many additional lints that can help prevent unidiomatic and inefficient
code. Install the subcommand with `cargo install clippy`.

Certain `clippy` lints may lead to false positives, or may not make sense in the
context of your task. In this repository, lints are allowed or denied based on
the presence of the `clippy` feature. So, you should invoke `cargo clippy` like
so:

```bash
$ cargo clippy --bin fizzbuzz --feature clippy
```

If you'd like to run `clippy` over all of the targets in the tree, awesome! Just
omit the `--bin` flag.

You may control lint warnings within your file by using `#[cfg_attr]`:

```rust
#[cfg_attr(feature = "clippy", allow(blacklisted_name))]
fn main() {
  // Normally, clippy warns when naming a variable "foo".
  let foo = "bar";
}
```

## Testing ##

If you are contributing a solution to this repository please try to include a test so we can verify correctness. Here are some guidelines:

* The testing code should demonstrate invocation and result of the completed task, not the task itself.
  * For example, if the task takes parameters, the `#[test]` should create the necessary values and pass them in.
  * Remember to test for failure, too. For example, if you're reading a file line by line, what happens if a file doesn't exist? One of Rust's biggest benefits is how it handles errors, show it off!

* Try to keep tests under 5 seconds on your machine. If your test takes longer than a few seconds, please add the `#[ignore]` attribute to it. You can still run the test with `cargo test -- --ignored`.

* Do not include tests which may cause system instability (e.g., forkbombs).

* Only talk to servers Rosetta code specifically directs you to.

* Do not download files unless that **is** the task. If that is the case make sure to delete the files afterwards.

* Do not execute anything unless that **is** the task. Do not execute anything as root.

* Do not depend on root privileges.

* Your testing code should be as simple as possible. Ideally it would look similar to this:

    ```rust
    #[test]
    fn test_file_exists {
        // ... Instantiate your necessary values (if there are enough to warrant it!) to pass in.
        let foo = 1;
        // ... Invoke your task and store the result.
        let result = my_task(foo);
        // ... Assert, unwrap, match, etc.
        assert_eq!(result, expected);
    }
    ```

* If you have multiple tests, you might want to include them in a test module:

    ```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn first_case() {
            // ...
        }

        #[test]
        fn second_case() {
            // ...
        }

        // ...
    }
    ```

[Cargo.toml]: ./Cargo.toml
[wiki]: https://rosettacode.org/wiki/Category:Rust
