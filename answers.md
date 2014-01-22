Title: Problem Set 1 Answers
Author: Nathan Typanski

1
=

````
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:24.0) Gecko/20131215 Firefox/24.0 Iceweasel/24.2.0
````

- `Mozilla/5.0`: From the 
  [Gecko User Agent String Reference](https://developer.mozilla.org/en-US/docs/Gecko_user_agent_string_reference):

  > Mozilla/5.0 is the general token that says the browser is Mozilla
  > compatible, and is common to almost every browser today.

  In other words, The browser works with certain standards that Mozilla also
  implements.
- `X11`: The browser is running within the 
  [X Window System](https://en.wikipedia.org/wiki/X_Window_System).
- `Linux x86_64`: 64-bit Linux host
- `rv:24.0`: Version number of Iceweasel.
- `Gecko/20131215`: Gecko rendering engine, from 2013-12-15.
- `Firefox/24.0`: Build comes from Firefox source code.
- `Iceweasel/24.2.0`: Iceweasel release (Debian rebranding of Firefox)

2
=

Rust thinks it is unsafe to modify a global variable like that because it
is especially dangerous to concurrency; you never know which thread is modifying
the variable at any given time, so the results are unpredictable.
