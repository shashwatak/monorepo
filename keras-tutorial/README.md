wasted a bunch of time getting tensorflow installed with latest versions of everything
- tf 2.19
- cuda 12.5

alas Waymo Open uses 2.12, which requires Cuda 11.9

wasted time trying to do all of this with UV directly on host, now I am using podman (should have started w/ container).


update, wasted more time, but got gpu, tf, and waymo open working.

- Cuda 11.9
- TF 2.11

working in uv

```
source .venv/bin/activate.fish
uv run waymo.py
```

