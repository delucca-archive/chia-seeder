# Architecture Decision Log

* [Why](#why)
* [What is an AD](#what-is-an-ad)
* [What is an ADR](#what-is-an-adr)
* [What is an ADL](#what-is-an-adl)
* [When should I write an ADR](#when-should-i-write-an-adr)
* [How to write an ADR](#how-to-write-an-adr)
* [Useful links](#useful-links)

## Why

Any system organically makes architecture decisions, but only some wrote those decisions down. Having a log of those decisions is the most efficient way to keep track of architectural changes and our application's current state. By doing so, new contributors can easily understand the underlying architecture of our system.

## What is an AD

An [architecture decision (AD)][architecture-decision-wikipedia] is a software design choice that addresses a functional or non-functional requirement that is architecturally significant.

## What is an ADR

An architecture decision record (ADR) captures a single AD, often done when writing personal notes or meeting minutes.

## What is an ADL

An architecture decision log (ADL) is the collection of all ADRs created and maintained for a particular project (or organization).

## When should I write an ADR

In a nutshell, you should always write an ADR if you're making a significant decision that impacts how engineers write software in this system. You can learn more about it [in this blog post][spotify-when-to-write-adr]

## How to write an ADR

Writing an ADR is a pretty complex topic, so we've created a [guide explaining how to write ADRs][write-adr-guide]. Check that guide to learn our architecture decision process.

## Useful links

* [When should I write an Architecture Decision Record][spotify-when-to-write-adr]
* [Thoughtworks' Technology Radar][thoughtworks-technogogy-radar]

[architecture-decision-wikipedia]: https://en.wikipedia.org/wiki/Architectural_decision
[spotify-when-to-write-adr]: https://engineering.atspotify.com/2020/04/14/when-should-i-write-an-architecture-decision-record/
[thoughtworks-technogogy-radar]: https://www.thoughtworks.com/radar/techniques/lightweight-architecture-decision-records
[write-adr-guide]: ../guides/writting-adrs.md