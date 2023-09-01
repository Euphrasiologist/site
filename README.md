Setting up Raspberry Pi as a website server from scratch

Probably the first thing to do (which is not the first thing I did)
is to set up port forwarding on the router. Firewall > port forwarding.

It's important to redirect port 80, and port 443.

Following this somewhat:
https://myhydropi.com/hosting-a-website-on-a-raspberry-pi/

Get a domain from namecheap. Namecheap is important as it
has dynamic DNS by default, and is easy to update via an API.

Add a CNAME and an A record. Now run the update_dns.py. We'l
put this on a cron job to update manually.

Now I'm setting up an nginx server, that's easy to do. Just 
follow any of the normal instructions for that. The hard bit 
is now setting up certificates:

https://www.centlinux.com/2020/07/install-ssl-tls-certificates-nginx-web-server.html

(comment out #ssl_ciphers PROFILE=SYSTEM;)

Pasting in the CA signed bit.

You are about to be asked to enter information that will be incorporated
into your certificate request.
What you are about to enter is what is called a Distinguished Name or a DN.
There are quite a few fields but you can leave some blank
For some fields there will be a default value,
If you enter '.', the field will be left blank.
-----
Country Name (2 letter code) [AU]:GB
State or Province Name (full name) [Some-State]:.
Locality Name (eg, city) []:Cambridge
Organization Name (eg, company) [Internet Widgits Pty Ltd]:.
Organizational Unit Name (eg, section) []:.
Common Name (e.g. server FQDN or YOUR name) []:Max Brown
Email Address []:max_brown1000@hotmail.co.uk

Please enter the following 'extra' attributes
to be sent with your certificate request
A challenge password []:Euphrasia!
An optional company name []:.


So I made one, it's here:
cat /etc/pki/nginx/nginx-01.csr

But to get this 'trusted' I gotta fuckin pay? You kidding? Just for https???
I am not doing that.


And after this, I am going to host a local website using nginx as a
proxy, which will shuttle through https traffic:

https://serverfault.com/questions/1104698/how-can-i-host-a-local-website-to-domain-using-nginx

And pray it works.
