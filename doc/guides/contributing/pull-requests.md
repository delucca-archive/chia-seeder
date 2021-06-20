# Pull Requests

* [Setting up your local environment](#setting-up-your-local-environment)
  * [Step 1: Fork](#step-1-fork)
  * [Step 2: Branch](#step-2-branch)
* [The process of making changes](#the-process-of-making-changes)
  * [Step 3: Pick an issue](#step-3-pick-an-issue)
  * [Step 4: Code](#step-4-code)
  * [Step 5: Commit](#step-5-commit)
  * [Step 6: Lint](#step-6-lint)
  * [Step 7: Rebase](#step-7-rebase)
  * [Step 8: Test](#step-8-test)
  * [Step 9: Push](#step-9-push)
* [Creating pull requests](#creating-pull-requests)
  * [Step 10: Opening the pull request](#step-10-opening-the-pull-request)
  * [Step 11: Discuss and update](#step-11-discuss-and-update)
* [Reviewing pull requests](#reviewing-pull-requests)
  * [Step 12: Review](#step-12-review)
* [Merging pull requests](#merging-pull-requests)
  * [Step 10: Landing](#step-10-landing)

## Setting up your local environment

To get started, you will need to have `git` installed locally. Depending on your operating system, there are also several other dependencies required. These are detailed in the [building guide][building-guide].

Once you have `git` and all [other dependencies][building-guide] installed, you can proceed to the following steps:

### Step 1: Fork

Fork the project [on Github][repository-url] and clone it locally:

```shell
git clone https://github.com/username/<repository-name>.git
cd <repository-name>
git remote add upstream <repository-git-url>
git fetch upstream
```

If you haven't done that already, configure `git` so that it knows who you are:

```shell
git config user.name "Your name"
git config user.email "Your e-mail"
```

### Step 2: Branch

As a best practice, keep your development environment as organized as possible. An excellent way to organize is to split your work into a new branch. These should also be created directly off the `main` branch:

```shell
git checkout -b my-branch -t upstream/main
```

> 💡 **TIP:** We suggest using the [gitflow pattern][gitflow-explanation] as a naming pattern for your branches

## The process of making changes

### Step 3: Pick an issue

You can pick an issue to solve directly from our [issues board][repository-issues-board]. There are many labels you can use that would help you choose the right fit for you.

If this is your first contribution, we suggest starting by filtering the [`good first issue` label][good-first-issue-label-url] since we add this label only to issues with a constrained scope that is easier to solve.

### Step 4: Code

While coding, we suggest following the best practices and keep in mind the [SOLID principles][solid-explanation]. Also, it would be best if you considered reading our [architecture decision log][architecture-decision-log] to understand the details regarding our system architecture.

### Step 5: Commit

As a best practice, try to keep your changes as logically grouped as possible within individual commits. We don't limit the number of commits in a particular pull request, so you can commit as many times as you want.

To make it easier while debugging, your commit messages should be standardized. We follow the [Conventional Commits Standard][conventional-commits].

### Step 6: Lint

This project follows a strict lint pattern. We've decided to do so to keep a standard in our codebase. A standardized codebase is easier to explore and understand. After finishing your issue, you can run the following snippet to lint your code, fixing any warnings and errors afterward:

```shell
🚧
```

### Step 7: Rebase

Before opening a pull request, it is a good practice to merge your work with the upstream `main` branch. We suggest using `rebase` for it. You can do so with the following:

```shell
git fetch upstream
git rebase upstream/main
```

### Step 8: Test 

Before opening a pull request, you must ensure that all previous tests are passing and that the new code you're adding is covered with tests. We've created a simple [writting tests guide][writting-tests-guide] that explains how you can add tests for your solved issue.

Read the [running tests guide][running-tests-guide] to understand how to execute this project's automated tests.

### Step 9: Push

Once you've finished your issue, rebased, linted, and all tests are passing, you can push your code to your remote branch. You can do so with the following command:

```shell
git push origin my-branch
```

## Creating pull requests

### Step 10: Opening the pull request

From GitHub, opening a new pull request will present you with a [pull request template][pull-request-template]. Please try to do your best at filling out the
details, but feel free to skip parts if you're not sure what to put.

Once opened, pull requests are usually reviewed within a few days.

### Step 11: Discuss and update

You will probably get feedback or requests for changes to your pull request. This is a big part of the submission process, so don't be discouraged! Some contributors may sign off on the pull request right away; others may have more detailed comments or feedback. This is a necessary part of the process to evaluate whether the changes are correct and necessary.

Make changes to an existing pull request, make the changes to your local branch, add a new commit with those changes, and push those to your fork. GitHub will automatically update the pull request.

```shell
git add my/changed/files
git commit
git push origin my-branch
```

It is also frequently necessary to synchronize your pull request with other changes that have landed in `main` by using `git rebase`:

```shell
git fetch --all
git rebase upstream/master
git push origin
```

## Reviewing pull requests

### Step 12: Review

It is an excellent practice to review your code, as well as other contributors' pull requests. We encourage everyone to contribute with pull requests by commenting and suggesting improvements.

While reviewing a pull request, we suggest the [Netlify's Feedback Ladder][netlify-feedback-ladder-explanation]. It is an easy standard to apply, improving our feedback structure while making pull requests.

Also, don't forget to **always** follow our [Code of Conduct][code-of-conduct] while interacting with others.

## Merging pull requests

### Step 10: Landing

To land, a pull request must be reviewed and approved by one Chia Seeder collaborator. After that, as long as there are no objections from other contributors, the pull request can be merged.

[building-guide]: ../building.md
[repository-url]: <repository-url>
[gitflow-explanation]: https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow
[repository-issues-board]: <repository-url>/issues
[good-first-issue-label-url]: <repository-url>/labels/good%20first%20issue
[solid-explanation]: https://en.wikipedia.org/wiki/SOLID
[architecture-decision-log]: ../adl
[conventional-commits]: https://www.conventionalcommits.org/en/v1.0.0/
[writting-tests-guide]: ../writting-tests.md
[running-tests-guide]: ../running-tests.md
[pull-request-template]: ../../../.github/PULL_REQUEST_TEMPLATE.md
[netlify-feedback-ladder-explanation]: https://www.netlify.com/blog/2020/03/05/feedback-ladders-how-we-encode-code-reviews-at-netlify/
[code-of-conduct]: ../../../CODE_OF_CONDUCT.md