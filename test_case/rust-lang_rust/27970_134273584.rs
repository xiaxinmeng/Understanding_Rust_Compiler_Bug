
       env    Functions marked with env as an MT-Safety issue access the
              environment with getenv(3) or similar, without any guards to
              ensure safety in the presence of concurrent modifications.

              We do not mark these functions as MT-Unsafe, however, because
              functions that modify the environment are all marked with
              const:env and regarded as unsafe.  Being unsafe, the latter
              are not to be called when multiple threads are running or
              asynchronous signals are enabled, and so the environment can
              be considered effectively constant in these contexts, which
              makes the former safe.
