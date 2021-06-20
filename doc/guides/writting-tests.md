# Writting tests for <repository-name>

* [What is a test](#what-is-a-test)
* [Types of tests we use](#types-of-tests-we-use)
  * [Acceptance tests](#acceptance-tests)
  * [Unit tests](#unit-tests)
  * [Mutation tests](#mutation-tests)
* [Best practices](#best-practices)
  * [General best practices](#general-best-practices)
  * [Acceptance tests best practices](#acceptance-tests-best-practices)
  * [Unit tests best practices](#unit-tests-best-practices)

## What is a test

A test aims to make it easier to identify if our system is solving all functional and non-functional requirements. Remember that your tests could be either **automated** or **not**, but you should always document them.

A non-automated test is not standard, but it is an excellent way to start, especially if you're a new member of our project. Like an automated test, you can write them down and perform them manually when you want.

Our project has all the required structures to execute automated tests. We encourage all contributors to write as many as possible but never forget [general best practices](#general-best-practices) while writing tests.

## Types of tests we use

### Acceptance tests

Those tests are responsible for ensuring that a given requirement is met. They can focus on a single unit or in the entire system, but acceptance tests always check if a given business requirement is met.

As a pattern, we suggest using [BDD][bdd-explanation] while writing acceptance tests.

Learn how to write acceptance tests [in our best practices section](#acceptance-tests-best-practices)

### Unit tests

A unit test isolates a given module and tests if a given scenario is met on that module. Opposing to acceptance tests, the unit tests typically focus on the **absence of a problem** instead of a feature's presence.

Learn how to write unit tests [in our best practices section](#unit-tests-best-practices)

### Mutation tests

In this type of test, mutations are inserted into our code, and we test if our current test coverage is being able to catch those mutations. We use this type of test to ensure that our tests are stable enough to prevent unwanted issues.

## Best practices

### General best practices

As a general rule, your tests should avoid testing implementation. While writing tests, keep in mind that your idea is to test the outcome without creating a dense couple between your test and the original code.

Mocking and other test tricks are often a good idea if they're isolated (like mocking a database, for example). Mocking a full service or a given business logic usually damages the test coverage and allows unwanted behavior.

### Acceptance tests best practices

We suggest writing an acceptance test scenario as a part of the requirements definition before coding. You can write those [in Gherkin][gherkin-reference] and attach your designed scenarios in the issue you're currently working on.

Try to keep your scenarios generic since you should reuse as many steps as you can. Also, it would be best if you stayed as close as a spoken language since it makes it easier to evaluate the coverage of a given requirement.

### Unit tests best practices

While writing unit tests, you should avoid duplicating and concurring with acceptance tests. The main idea of unit tests is to prevent not wanted scenarios. Instead of writing `it should create a user`, your tests should be like `it should not create a user without an e-mail`. This may seem like a slight difference, but it has a massive impact on our test structure and reliability.

[bdd-explanation]: https://cucumber.io/docs/bdd/
[gherkin-reference]: https://cucumber.io/docs/gherkin/reference/