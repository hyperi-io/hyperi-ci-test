# .ci-negative - DELIBERATE PLANTED FAILURES. DO NOT FIX.

> DO-NOT-FIX. The files in this directory are DELIBERATE. They exist to
> make CI fail in a controlled way. Repairing them defeats the test.

Everything under `.ci-negative/` is a negative E2E case for the hyperi-ci
fleet sweep (branch-mode plan, decision 9). These are NOT bugs in the
fixture and NOT scaffold drift - do NOT "helpfully" fix, format, lint,
patch, or delete them. The boy-scout / fix-as-you-go rule that applies to
the rest of the `ci-test-*` fleet explicitly EXCLUDES this directory.

## How a case is shaped

Each case is data on `main`, never a long-lived bad branch:

- `<case>.patch` - the diff that plants the failure, applied to `main`.
- `<case>.yaml` - what the run must do: `expect`, `stage`, `reason`.

`hyperi-ci/scripts/negative-cases.py` applies the patch to an ephemeral
`expect-fail/<case>` branch, opens a pull request, asserts the run fails
at the declared stage for the declared reason, then closes the PR and
deletes the branch. Wrong stage is a failure. Green is a failure.

The patches carry the same contract in their header comment. That header
is the human-readable copy; the `.yaml` is the one the runner reads.

Two things the run depends on, both easy to get wrong:

- The branch commit has to be a release-worthy type (`fix:`), AND the run
  has to be a `pull_request` one. hyperi-ci skips the quality job for
  non-bumping commits, and its plan job skips it for a push to a non-main
  branch - a bare branch push goes green having run no gate at all. The
  `lint-manifests` case is the exception: that job is the fixture's own
  and is not run-checks gated.
- `hyperi-ci lint-manifests` gates on kubeconform only. kube-linter and
  checkov run beside it as advisories, so a best-practice finding will
  not fail the stage.

## Cases

| Case             | Stage          | Reason      | Proves                                          |
|------------------|----------------|-------------|-------------------------------------------------|
| `hadolint-error` | quality        | hadolint    | the container gate blocks on DL3004             |
| `schema-invalid` | lint-manifests | kubeconform | `lint-manifests` blocks on an invalid manifest  |

- `hadolint-error.patch` adds `RUN sudo apt-get update` to the Dockerfile.
  hadolint flags DL3004 (do not use sudo) at ERROR severity, which fails
  the container gate inside `hyperi-ci run quality`.
- `schema-invalid.patch` adds a Deployment manifest with `replicas: "three"`
  (a string where the schema requires an integer). kubeconform rejects it,
  failing the `hyperi-ci lint-manifests` gate.
