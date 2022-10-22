### Issue
Using multiple configuration time constants with the `log_with_type` function results in inconsistent behaviour.
The function does not return anything however `fetch_all_logs` does return the expected data.

When a config-time-constant is set to a constant variable in the contract, and its NOT commented out, the inconsistent behaviour appears;
however, when the const is commented out, the inconsistent behaviour disappears and logging seems to work as expected.

Using only 1 configuration time constant does not result in any failures in the test, using 2 constants seems to sometimes fail, 4 seems to always fail.
The name of the constants, the type, the value, or the order in which the constants are declared, do not seem to have an effect on the behaviour.

The struct which is being logged does seem to affect the behaviour, the number of fields and the type of the fields seems to affect the behaviour.
We tried `u64`s, `str`s, `Identity`s, we could not pinpoint how they affected the behaviour as the test failed or passed seemingly randomly

To reproduce the bug, run
```bash
    forc build
    cargo test
```