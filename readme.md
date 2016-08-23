# pipeline

An attempt to create a statically declared pipeline.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [General Idea](#general-idea)
- [Strong Typing](#strong-typing)
- [Supplemental Type Information](#supplemental-type-information)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## General Idea

I often want to create a pipeline of events that flow from one to the next. Typically this is part
of a release process of some sort, but I could imagine this being useful for other things as well.
I often have to resort to an amalgamation of tools, shell scripts, or Jenkins. I want something that
can meet all of these demands. I also want to design a system that is both typed and declarative.

For the typed aspect, it really boils down to each "phase" of the pipeline accepting some typed
input and outputting a typed result. The declarative aspect is that each pipeline "job" should be
able to be defined using something like YAML and not require complex programming or a custom DSL.


## Strong Typing

I would like the typing of the plugins' I/O to be strongly typed. The end goal is to be able to safely
plug phases together to form the pipeline. However this typing is done, there are some goals to meet

+ multiple phases can output the same types
+ phase input should not be directly tied to a single's phases output explicitly
+ a "non-breaking" change in the output-type of an upstream phase should not break a downstream phase

This creates some interesting problems. The first is the idea of named types. Let's say that Phase A
outputs an object that looks like

```json
{
  "type": "rpm",
  "artifact": {
    "version": "0.1.2",
    "download-from": "http://some.rpm-server.com/rpm/my-artifact-0-1-2.rpm"
  }
}
```

We may be inclined to _name_ the output type of this something like `PackagingInfo`. But what happens
when Phase B is registered with the pipeline using the same named type but outputting Docker images
instead? Presumably they would use the same named type. Some questions come up

+ where is this type defined?
+ who owns this type?
+ does changing this type involve coordination across multiple phases?
+ how is this type versioned?
+ does a phase need to specify a version of the input they require?

Wow this is turning into a mess. Let's forget named types for the moment. What may be more flexible in
the long-run is to use some sort of structural, duck-typing. This will be similar to Go's interfaces.

+ request
  + required vs optional fields
  + field names + types
  + object types are defined as sub-messages (no named types)
+ response
  + field names + types


An example of what this might look like for a phase output for the above example

```yaml
type: string
artifact: structural
  version: string
  download-from: string
```

and for phase input

```yaml
type:
  - _type: string
  - _required: true
artifact:
  - _type: structural
  - required: true
  version:
    - _type: string
    - _required: false
  download-from:
    - _type: string
    - _required: true
```

The input requirements can additionally specify, aside form the types, whether or not the field is
required or not. It may be worth defining something similar in the output although it may make just
as much sense to say that everything is optional.

## Supplemental Type Information

Phases should fit together via inputs and outputs. However it also makes sense that the pipeline definition
could also define some "default" inputs for phases. This allows phases to be setup as "pure functions" that
only work with input and output and do not have any other sort of "configuration" mechanism.
