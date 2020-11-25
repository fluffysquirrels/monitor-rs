# `shell_check` timeout notes

* Crate process_control. Terminates the process from another thread,
  doesn't support merging stdout and stderr.
* Various solutions suggested here:
  https://stackoverflow.com/questions/282176/waitpid-equivalent-with-timeout
* Poll for completion?
* sigtimedwait for SIGCHLD, but signals are still nasty and need masking on all threads
  https://man7.org/linux/man-pages/man2/sigtimedwait.2.html
* signalfd for SIGCHLD, but signals are still nasty and need masking on all threads, Linux only
    * See https://ldpreload.com/blog/signalfd-is-useless
* `pidfd_open` (requires Linux 5.3, I'm on 4.15.0)
    * Crate for this: https://docs.rs/mio-pidfd/0.1.1/mio_pidfd/index.html
* wait without timeout and get interrupted by SIGALRM
* kill -9 from another thread on reaching timeout
* https://docs.rs/signal-hook/0.1.16/signal_hook/index.html
* nix crate support for signals:
    * https://docs.rs/nix/0.19.0/nix/sys/signal/index.html
    * https://docs.rs/nix/0.19.0/nix/sys/signalfd/index.html
