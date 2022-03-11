<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi, <span id="name-small">I'm Arthur</span></h1>

        <div id="bottom">
          <p>Born &amp; raised in Silicon Valley; currently studying computer science at UC Irvine, class of 2023. Also
             in between summers at <span class="strikethrough">Facebook</span> Meta. I'm broadly interested in programming language design &amp; implementation, operating systems,
             and systems security. Among other things, networking, computer architecture, and electrical/computer engineering are also cool.
          </p>

          <div id="contact">
            <p>To learn more about me or get in touch, feel free to check out:</p>

            <div class="button-group">
              <Button class="navy" v-for="(method, index) in contactMethods" :key="index" :title="method.name" :destination="method.link" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </BlankSection>

  <Section name="highlights" title="Highlights">
    <List :items="highlights" />
  </Section>

  <EntryList name="current-work" title="Things I'm Working On" :entries="currentWork">
      <p class="description">I tend to switch gears pretty frequently, so this section will tend to change just as frequently</p>
  </EntryList>

  <EntryList name="past-work" title="Things I've Worked On" :entries="pastWork" />

  <Section name="resume-dump" title="Resume Dump">
    <p class="description">A wall of text (somewhat) repurposed from my resume</p>
    <List :items="resumeDump" />
  </Section>

  <div class="footer">
    <p class="copyright navy">Copyright (c) 2022 Arthur Lafrance</p>
  </div>
</template>

<script>
import List from './components/List.vue'
import Button from './components/Button.vue'
import BlankSection from './components/BlankSection.vue'
import Section from './components/Section.vue'
import EntryList from './components/EntryList.vue'

export default {
  name: 'App',
  components: {
    List,
    Button,
    BlankSection,
    Section,
    EntryList
  },
  data: function() {
    return {
      contactMethods: [
        { name: "Email", link: "mailto:lafrancearthur@gmail.com" },
        { name: "GitHub", link: "https://www.github.com/a-lafrance" },
        { name: "LinkedIn", link: "https://www.linkedin.com/in/arthurlafrance" },
      ],
      highlights: [
        { text: "Foodpool launched in closed alpha in March 2022!", subitems: [] },
        { text: "I'll be returning to Meta for summer 2022, where I'm hoping to continue working with PLs & compilers.", subitems: [] },
        { text: "As of late 2021, I've been exploring the use of Rust to enforce safety in OS design with Prof. Burtsev's Mars Group.", subitems: [] },
        { text: "I spent summer 2021 at Meta, where I worked on compilers, profiling, & infrastructure.", subitems: [] },
        { text: "During summer 2020, I interned with Centric Software's iOS development team.", subitems: [] },
        { text: "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack.", subitems: [] }
      ],
      currentWork: [
        {
          title: "Foodpool",
          subtitles: [],
          description: `Foodpool is a food delivery startup my roommates and I created to make food delivery affordable for everyone,
          centered around the idea of "foodpooling": carpooling, but with food delivery. Over the last 6 months or so, we've been ironing
          out the kinks in the system and writing tons of code (Swift for iOS and Rust for our backend stack). It's great to see our work
          start to pay off too: we just launched in closed alpha in March 2022!`,
          link: "https://www.foodpool.app"
        },
        {
          title: "lfc",
          subtitles: [],
          description: `Let's face it: C doesn't give you much out of the box. If you need to use most of the conveniences provided by
          modern languages, you have to write them yourself. To that end, lfc is where I write them myself: it almost feels like a running
          "standard library" that I maintain on an as-need basis. Honestly, that's one of the fun parts about C: not only the ability, but the
          need to think about manually implementing this stuff that you can usually take for granted.`,
          link: "https://www.github.com/arthurlafrance/lfc.git"
        },
        {
          title: "...and others",
          subtitles: [],
          description: `As has always been the case for me, there are a couple projects taking up my spare time, that may or may not end up
          seeing the light of day. If they do, great! They'll make it onto this list. If not, good thing I didn't put them here prematurely.`
        }
      ],
      pastWork: [
        {
          title: "Tapestry",
          subtitles: [],
          description: `In ICS 33, the last UCI intro to CS class, Prof. Pattis used "memory diagrams" drawn with ASCII art to teach us
          about the internal memory representations of certain Python data structures. Inspired by this technique, my two good friends and I
          decided to create a tool that dynamically generates these diagrams at HackUCI 2020. We created Tapestry, a web-based code visualization
          tool capable of generating these memory diagrams for arbitrary Python programs. We even spent the next few months further developing
          the tool, and who knows, it might make its way into the "Current Work" section at some point.`,
          link: "https://www.github.com/tapestrylearn/Diagrammer"
        }
      ],
      resumeDump: [
        {
          text: "I write quite a bit of systems code, including:",
          subitems: [
            "I've used Rust for static analysis, and to write operating systems, web servers, and personal projects–I've even contributed to Rust in Rust.",
            "I've used C and C++ for compiler development, graph alignment research, and (of course) personal projects",
            "I'd also like to pick up, among other things: Golang, Zig, RISC-V"
          ]
        },
        {
          text: "I've taken lots of cool classes, but my favorites include:",
          subitems: [
            "ICS 53: Principles in System Design",
            "CS 142A: Compilers & Interpreters",
            "CS 143A: Principles of Operating Systems",
            "CS 162: Formal Languages & Automata"
          ]
        }
      ]
    }
  }
}
</script>

<style>
body {
  margin: 0;
  background-color: rgb(250, 250, 250);
}

.navy {
  color: rgb(0, 60, 120);
}
</style>

<style scoped>
h1 {
  font-size: 30pt;
  font-weight: normal;
}

#name-small {
  font-size:  /*24*/30pt;
  /* font-weight: 100; */
}

.button-group {
  display: flex;
  flex-direction: row;

  padding-left: 2vw;
}

#bottom {
  display: flex;
  flex-direction: row;
}

#intro-content {
  display: flex;
}

#bio {
  padding-right: 1vw;
}

#intro-content p {
  font-size: 12pt;
}

#contact {
  margin-left: 2vw;
}

.copyright {
  padding-left: 3vw;

  color: rgb(50, 50, 50);
  font-family: "Helvetica", sans-serif;
  font-weight: 100;
}

.strikethrough {
    text-decoration: line-through;
}
</style>
