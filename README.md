# NOT PRODUCTION CODE

This code is crappy, it's just a quick hack to see if I got the idea.
Let's discuss and see if I'm on the right track :)

If you give it a run, it should produce md compatible output from the example folder.
Toy around with it and see if it does what you expect.

Also, keep in mind that the landing.md files are needed,
because I need to link to _something_, but I can't know what's right.
If a landing.md doesn't exist, a title will be guessed.

---

Currently, the example folder results in this:

```markdown
# SUMMARY

- [Chapter 1 title parsed from markdown file](./chapter_1/landing.md)
    - [Section 1.1 title](./chapter_1/section_1_1.md)
    - [Section 1.2 title](./chapter_1/section_1_2.md)
    - [Section 1.3 title](./chapter_1/section_1_3.md)
- [Chapter 2 title](./chapter_2/landing.md)
    - [Section 2.1 title](./chapter_2/section_2_1.md)
```
