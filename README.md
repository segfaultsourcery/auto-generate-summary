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

- [Landscaping On The Moon](./chapter_1/landing.md)
    - [If Dogs Had More Knees](./chapter_1/sub_chapter_2/landing.md)
    - [Bees Do They Eat Lunch](./chapter_1/sub_chapter_1/landing.md)
    - [Empty Folder](./chapter_1/empty_folder/landing.md)
- [My Favorite Boat Flavor](./chapter_2/landing.md)
    - [A Deep Dive Into Nonsense](./chapter_2/sub_chapter_1/landing.md)
```