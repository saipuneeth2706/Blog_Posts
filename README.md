## What is the basic idea
### What all i need in my blog post website
- Blogs (ofc)
- Auth for users to signin
    - users can comment like the blogs and also add their own blogs
- only google auth
    - i will be fetching only the users full name and email with pfp as well
- the url would be "blogs.saipuneeth.me" which i already own
- i will be adding this link on my portfolio website "saipuneeth.me"
- the landing page would contain some featured blogs and some featured authors as well
- markdown support for blogs so that users can post code blocks as well
- support auto save and drafting while writing blogs
- also support for image uploads as well
- how much time will user spent reading should be mentioned and it will be automatically generated
- lets add some tags/topics
    - linux
    - latest tech gadgets / laptops / phones
    - AI
    - General stuff about IT industry
- Admin Dashboard as well if possible (planned for later)
## the main tech stack to use
### frontend
- next js
- icon libraries for icons in the page, maybe lucide icons
### backend
- rust
- axum framework for rust backend development
- diesel orm for db management
### database
- postgresql which would be in a docker container with an opened port 5432 which is default
## No of pages and what are they
- there are total 6 pages in the website in which all the pages have same header and footer
### landing page
![[readme_res/Pasted image 20260424202550.png]]

### auth page
![[readme_res/Pasted image 20260424202822.png]]
### authors page
![[readme_res/Pasted image 20260424202838.png]]

### blogs page
![[readme_res/Pasted image 20260424202854.png]]
### blogs full screen page
![[readme_res/Pasted image 20260424202915.png]]

### write blogs page
![[readme_res/Pasted image 20260424202934.png]]