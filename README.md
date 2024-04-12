# Responder

[r.jtw.sh](https://r.jtw.sh/)

**Easily create and share Proof of Concepts in HTML, JavaScript, etc. with custom headers, all via query parameters**

<div align=center>
  [<img width="775px" src="https://github.com/JorianWoltjer/responder/assets/26067369/48d2f5f4-afc0-4116-8673-a6abd639bad0" alt="Screenshot of default main web UI">](https://r.jtw.sh/)
</div>

Ever wanted to create an easy-to-share Proof of Concept, but realized your exploit need a special header here, another special status code there, and oh yeah, you forgot *Cross-Origin Resource Sharing* again. No more!  
With Responder, you have a super quick **web UI** that generates a permanent URL that will always respond correctly. **Query parameters** tell it what body to use, what headers to set, and what status code to use. This way, nothing is stored and nothing can be lost, you just share the URL. With the quick preview options, you can even use this **while developing** your exploit.

## Features

* **Respond on any path**: Every pathname will lead to the same parser and respond in the expected way. By setting a filename extension like `.html`, it automatically guesses the `Content-Type` header if it wasn't overwritten manually.
* **Set body data**: Use the `?body=` parameter to set an content body, or use the shorthand `?body.b64=` to decode from raw Base64 instead.
* **Set exact response headers**: Use the `?h[Header-Name]=` parameter multiple times to set any amount of headers for the response. There are some shorthands like `?h[ct]=` for `Content-Type`, or `?h[l]` for Location. Lastly, the `?cors` paramter alone will set all wildcard [Cross-Origin Resource Sharing](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) headers to make sure cross-site requests can read the content.
* **Status code**: Use the `?status=` parameter to set the [response status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
* **Gist fetching**: For large bodies that won't fit in a URL, you can [create a gist](https://gist.github.com/) and put the URL in the `?gist=` parameter. This will fetch the gists raw content and return it as the body, with any custom headers/status you set.

## FAQ

1. **How can I shorten the URL?**: If your target supports it, you could simply redirect/rewrite from your short domain to a crafted URL on this tool. Otherwise, you can always fall back to a simple page yourself like with PHP.
2. **How do I see exactly what headers my response is giving?**: Send a request to the url with `curl -D - 'https://r.jtw.sh/...'`, or use a proxy like Burp Suite to view the raw traffic.

If you have any other questions, feel free to ask in an [Issue](https://github.com/JorianWoltjer/responder/issues).

## Privacy & Self-Hosting

> [!WARNING]  
> Because this tool is hosted on my own domain, I will be able to view any traffic going to and from my server. Keep this in mind when creating PoC's for real-world scenarios and consider self-hosting it on a server you control.

Build and run the repository like this:

```bash
cd frontend/
npm i
npm run build

cd ../
cargo build --release

./target/release/responder
```

**Be careful on which domain you run this tool**. The domain must not host any other applications that rely only on SameSite for CSRF protection ([read more](https://book.jorianwoltjer.com/web/cross-site-request-forgery-csrf)). This application is vulnerable to XSS by design, so it may be abused to send same-site requests with authentication.
