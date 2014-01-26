ps1
===

This is Problem Set 1 for [Rust Operating Systems Class](http://rust-class.org/pages/ps1.html) (cs4414).

- This top-level folder contains actual problems for the problem set.
    - The following problems are completed:
        - **Problem 1**. 
            
            > (write your answer in answers.md, clearly marked as 1.)
            > Copy the User-Agent string reported by your browser. Explain as many
            > of the things in that string as you can. 
        - **Problem 2**. 
            
            > (2. in answers.md) Speculate on why Rust thinks it is
            > unsafe to modify a global variable [...]. 
        - [**Problem 3**](joiner.rs). 
            
            > (modify zhttpto.rs) Modify the server so it maintains a count of
            > the number of requests, and adds a message to the response that
            > includes a count of the number of requests. You should see the
            > number increase each time your reload the page.
        - [**Problem 4**](https://github.com/nathantypanski/ps1/blob/37cf4f6df7cb237efaf57660e7bfbd2189c489d9/zhttpto.rs).

            > Even with the counter, our web server is not very useful! For the
            > rest of this assignment, your goal is to modify the web server to
            > serve requested static pages.

            > If the first line of the incoming request matches GET /<path>
            > HTTP/1.1 where <path> is a non-empty file system path. Your
            > server should respond by sending the contents of the file
            > <cwd>/<path> where <cwd> is the current working directory (where
            > you started the web server) in the response.

            > If the incoming request contains an empty path, your server
            > should respond as it did in the previous problem.
        - [**Problem 5**](https://github.com/nathantypanski/ps1/blob/901f514e9909b548443a834e9abbee0a2bf54d6e/zhttpto.rs).

            > Modify the server to only serve files that have .html extensions
            > and that are in the directory where the server is started
            > (including any subirectories of that directory). If a requests
            > asks for a file that is not permitted, your server should respond
            > with an error page (this should use the 403 HTTP response code in
            > place of the 200 in the normal response; if you are feeling
            > humorous, you can use 418 instead.).
    - [answers](answers.md) has written answers to questions.
    - [zhttpto](zhttpto.rs) has Rust solutions to the Zhttpto web server 
      programming problems.
- The folder [tutorial](tutorial) contains examples worked over during the tutorial.
