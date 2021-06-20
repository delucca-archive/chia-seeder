# Issues

* [Asking for general help](#asking-for-general-help)
* [Submitting a bug report](#submitting-a-bug-report)
* [Proposing new features](#proposing-new-features)

## Asking for general help

To avoid mixing issues and feature requests with general questions and help requests, we've enabled the [discussions tab][discussions-tab]. There, you can start new discussions in the `Q&A` category, explaining your question. A member of our community will gladly answer your question there.

## Submitting a bug report

It's complicated to ensure quality and prevent unexpected bugs. There are so many corner cases that it is impossible to avoid bugs from happening. So, one of the most important contributions you can give us is reporting issues and bugs.

Reporting bugs is not a simple task. It is not a simple "feedback form" where you can type what happened, and we will evaluate. While reporting bugs, you need to keep in mind that **we need to reproduce your issue**, so we need as much information you can give us.

The two most important pieces of information we need to evaluate the report properly are a description of the behavior you are seeing and a simple test case we can use to recreate the problem independently. If we cannot replay the issue, it becomes impossible for us to fix it.

To rule out the possibility of bugs introduced by userland code, test cases should be limited, as much as possible, to use the minimum dependencies as possible. See [How to create a Minimal, Complete, and Verifiable example][creating-mcve].

To make it easier for our users, we [have a `bug-report` template][template-bug-report] that you can use while reporting a bug (you can select it while writing your issue). Please, fill all the sections while doing so.

## Proposing new features

Our system is only helpful if it does what our users want. A considerable part of our job is to prioritize the most relevant feature requests to code.

You can contribute to us by adding new issues with [the `feature-request` template][template-feature-request], where you can explain your idea and what you want our system to solve not currently being solved yet.

Although our primary focus is on our users, keep in mind that some proposed features could be either too complex or not even related to our primary domain. So, creating a feature request has no guarantee that our team would develop it.

[discussions-tab]: ./discussions
[creating-mcve]: https://stackoverflow.com/help/mcve
[template-bug-report]: .github/ISSUE_TEMPLATE/bug-report.md