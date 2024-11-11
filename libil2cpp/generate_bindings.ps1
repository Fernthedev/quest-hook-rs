#!/bin/pwsh


bindgen wrapper.h -o bindings.rs --wrap-unsafe-ops --sort-semantically