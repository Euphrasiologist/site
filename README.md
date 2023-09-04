# My public site

Currently running on a tiny Raspberry Pi. Read on if you'd like to know how I did this, as I found it pretty hard to find a good tutorial on how to do this.

It's running a Rust web server on a simple custom static site. Nothing particularly exciting!

First up, useful links which you may want to refer to at several points:

- https://myhydropi.com/hosting-a-website-on-a-raspberry-pi/
- https://www.centlinux.com/2020/07/install-ssl-tls-certificates-nginx-web-server.html
- https://serverfault.com/questions/1104698/how-can-i-host-a-local-website-to-domain-using-nginx

You don't have to do everything strictly in this order, but I would.

## Set up port forwarding on your home router

Probably the first thing to do (which is not the first thing I did) is to set up port forwarding on the router. To get to your router type in your browser address bar the IP address which starts with `192.168...`. Then explore on that, each hub brand is different. This might be under a firewall tab, but you are looking for port forwarding.

It's important to redirect both port 80, and port 443. Port 80 is the default HTTP port, and port 443 is for the SSL/TLS certificate layer, which is for HTTPS (secure HTTP) requests.

## Get a domain name

I strongly recommend namecheap, as it seemed to have a nice API for dynamic DNS, which meant I didn't have to download some old/weird dynamic DNS tool. If you don't know what dynamic DNS is, it means you are able to update your DNS with the correct IP address if it changes. Residential (i.e. non business) IP addresses are prone to changing frequently, which is annoying. So it's likely you will have to deal with this.

For the IP addresses for this bit, just put in your current public IP address. It'll update on the script below periodically but you need to put something in there initially.

On namecheap (or another provider, but as I said, namecheap is easy) add:
- CNAME
- A record(s), one @ and one 'www'

Be sure to make these dynamic DNS enabled. You'll get a code which you can run this script:

```python
!/usr/bin/env python

# run via a cron job.

from urllib2 import urlopen

# this is for the @ A record
urlopen("https://dynamicdns.park-your-domain.com/update?"
        "host={}"
        "&domain={}"
        "&password={}"
        .format("@", "<DOMAIN NAME>", "<PASSWORD>"))

# and this for the www record
urlopen("https://dynamicdns.park-your-domain.com/update?"
        "host={}"
        "&domain={}"
        "&password={}"
        .format("www", "<DOMAIN NAME>", "<PASSWORD>"))
```

You will need to run this script frequently, hourly-daily I would think. So whack it on a cron-job on your PI machine. `crontab -e` is your friend. For reference, I did this:

- https://crontab.guru/#\*/30\_*\_*\_*\_*

## Set up `nginx`

Now it's time to get a little knarly. This is probably the bit I knew least about and was kind of stabbing in the dark a bit. `nginx` is a cool bit of software which in our case is going to act as a middle layer server. It will take the public requests from the internet, and redirect them on our system to the correct place. It's apparently super optimised, and recommended for this type of thing - but don't quote me on that. Eventually we'll have a local Rust server running, which nginx will point to. But not yet.

Download `nginx`. There's a billion websites that tell you how to do it. I think it's `sudo apt-get nginx` for the Pi. Initialise it (again many tutorials on that - the `sudo systemctl start/stop/status nginx` commands are helpful/essential), and then you will have to hack on the config file to make it work on your system. Once your hack works, you should get an empty nginx error message whenever you navigate to your home page on the internet.

If you want HTTPS support, which for us will manifest itself as a 'are you sure you want to proceed to this website, it might be unsafe?' webpage when people go to the HTTPS request for your website, this requires a bit more work. It's up to you if you'd like to do this or not but be prepared to be a little frustrated. This website (https://www.centlinux.com/2020/07/install-ssl-tls-certificates-nginx-web-server.html) worked very well for me. Essentially, you need to create a key and certificate (self assinged as you will be making them), and refer to them in your `nginx` configuration.

I'm not gonna paste here my whole config caus it's probably confidential. The critical bit from my experience is the server block, inside the http block. You will want something like this:

```txt
# other bits up here

http {
  server {
    # you want to listen on port 80 for incoming HTTP requests
    listen 80;
    # and on port 443 for HTTPS requests (if you set that up)
    listen 443 ssl;

	  # your server name
    server_name  maxcarterbrown.org;
    root         /usr/share/nginx/html;
    
    # important bits if you want HTTPS
    ssl_certificate "path to self generated certificate";
    ssl_certificate_key "path to private key";

    # v important to redirect HTTPS -> HTTP
    location / {
	    # this is the key bit to re-direct
	    proxy_pass	http://127.0.0.1:8080;
    }
}

# other bits down here
```

If you desperately want your website to be 'properly' trusted with HTTPS, you need to pay another company some non-trivial amount of money to put your website on a secure list as far as I can tell. I didn't like the idea of this, and wanted to be cheap, so I have no experience of this. I think once you pay the money, the provider will send you an email with your key and certificate which you will reference much in the same way as your self assigned ones. I think.

## The Rust server (finally)

Now it's time to make your Rust server. Or any server! A dirty quick and cheap one is to create an index.html in your local directory, then run a small python server (note, not production ready, if a Pi website can ever be considered production ready):

```bash
echo "<h1>Hello world!</h1>" >> index.html
python3 -m http.server 8080 --bind 127.0.0.1
```

 Check out the code for my implementation, but the important bit is that you point your server to the localhost (127.0.0.1:8080 - I don't think it *has* to be port 8080 but make sure your `nginx` config and your server code match!). You can't do port 80, caus that requires `sudo` access. I checked this, don't worry.

## Run the server, check the browser

Now you can run your server on your little Pi, and go to your domain name, and it should display your website. Please note, this set up has little to no security, if someone mean attacks your website there isn't much you can do. So I hope nobody does that. This isn't AWS or anything.

Lastly, have fun! I hope this short guide has been somewhat comprehensive. I learned a lot about how the web/websites work doing this. I've probably left enough out that you'll need to hack and be a little frustrated, but you should be able to get it done. If not, please let me know and I'll add in some more notes. I'm reasonably active on GitHub.