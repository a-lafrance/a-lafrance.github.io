<template>
  <List :items="this.items" />
  <Link text="hi there" href="http://www.arthurlafrance.com" />
  <Button title="GitHub" destination="https://www.github.com/arthurlafrance" />
  <Entry :entry="entry" />

  <Section name="test">
    This is a test section
  </Section>

  <EntryList name="test" title="Entry List" :entries="entries" />
</template>

<script>
import List from './components/List.vue'
import Link from './components/Link.vue'
import Button from './components/Button.vue'
import Entry from './components/Entry.vue'
import Section from './components/Section.vue'
import EntryList from './components/EntryList.vue'

export default {
  name: 'App',
  components: {
    List,
    Link,
    Button,
    Entry,
    Section,
    EntryList
  },
  data: function() {
    return {
      items: [
        { id: 1, text: 'hello, world' },
        { id: 2, text: 'asg' },
        { id: 3, text: 'acya' },
      ],
      entry: {
        title: 'test entry',
        subtitles: [
          { id: 1, text: 'subtitle 1' },
          { id: 2, text: 'subtitle 2' }
        ],
        description: 'this is a test entry, nothing to see here',
        link: 'https://linkedin.com/in/arthurlafrance'
      },
      entries: [
        {
          title: 'test entry',
          subtitles: [
            { id: 1, text: 'subtitle 1' },
            { id: 2, text: 'subtitle 2' }
          ],
          description: 'this is a test entry, nothing to see here',
          link: 'https://linkedin.com/in/arthurlafrance'
        },
        {
          title: 'test entry 2',
          subtitles: [
            { id: 1, text: 'subtitle 3' },
            { id: 2, text: 'subtitle 4' }
          ],
          description: 'this is another test entry, nothing to see here either',
        }
      ]
    }
  }
}
</script>

<style>

</style>
