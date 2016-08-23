# pipeline

An attempt to create a distributed, statically defined build pipeline.

## General Idea

I often want to create a pipeline of events that flow from one to the next. Typically this is part
of a release process of some sort, but I could imagine this being useful for other things as well.
I often have to resort to an amalgamation of tools, shell scripts, or Jenkins. I want something that
can meet all of these demands. I also want to design a system that is both typed and declarative.

For the typed aspect, it really boils down to each "phase" of the pipeline accepting some typed
input and outputting a typed result. The declarative aspect is that each pipeline "job" should be
able to be defined using something like YAML and not require complex programming or a custom DSL.
