<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi,</h1>
        <h3 class="navy">I'm Arthur</h3>

        <div id="bottom">
          <p>Born &amp; raised in Silicon Valley; currently studying computer science at UC Irvine, class of 2023. Also
             in between summers at <span class="strikethrough">Facebook</span> Meta. I'm broadly interested in compilers, programming language design, &amp; computer
             architecture. Operating systems, networking, &amp; electrical/computer engineering are also cool.
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
      <p class="description">Given the volatile nature of my projects at this point, I'm sure this list will be in a pretty constant state of flux.</p>
  </EntryList>

  <EntryList name="past-work" title="Things I've Worked On" :entries="pastWork" />

  <Section name="resume-dump" title="Resume Dump">
    <p class="description">A giant wall of text more or less regurgitated from my resume</p>
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
        { name: "GitHub", link: "https://www.github.com/arthurlafrance" },
        { name: "LinkedIn", link: "https://www.linkedin.com/in/arthurlafrance" },
      ],
      highlights: [
        { text: "I'll be returning to Facebook for summer 2022, likely (hopefully) to continue working with PLs & compilers.", subitems: [] },
        { text: "I've started research with Prof. Burtsev's Mars Group, exploring Rust as a mechanism for enforcing OS safety invariants.", subitems: [] },
        { text: "I spent summer 2021 at Facebook, where I worked on compilers, profiling, & infrastructure.", subitems: [] },
        { text: "During summer 2020, I interned with Centric Software's iOS development team.", subitems: [] },
        { text: "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack.", subitems: [] }
      ],
      currentWork: [
        {
          title: "Foodpool",
          subtitles: [],
          description: `My roommates and I recently started Foodpool, a food delivery startup aiming to increase the efficiency of delivery
          by pooling multiple orders into a single trip. For the past few months, we've been working out the kinks in the idea, testing the
          system ourselves, and (of course) writing lots of code; we're looking forward to all of our work coming to fruition in the near future.
          Specifically, I've spent my time architecting the tech required to support our system, and writing code in Swift (iOS) and Rust, with
          Tokio & Axum (backend). It's been a pleasure to finally be able to use Rust in a production environment.`,
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
          text: "Undergraduate Researcher, UC Irvine, November 2021 - present",
          subitems: [
            `With Prof. Burtsev and the Mars Group on RedLeaf, I'm exploring the use of Rust language features to enforce safety & fault isolation invariants in
            operating system design; specifically, Rust's memory safety and linear type system allow us to do some interesting things in OS design.`
          ]
        },
        {
          text: "Engineering Lead, UCI Student Center & Event Services, November 2021 - present",
          subitems: [
            `After transitioning into less of a developer role, my day-to-day consists mostly of software architecture, code review, and developer mentorship, focusing on the big
            picture processes that facilitate developer success and bridging the gap between developers and non-technical stakeholders.`
          ]
        },
        {
          text: "Co-Founder & CTO, Foodpool, September 2021 - present",
          subitems: [
            `As I mentioned above, my roommates and I started Foodpool, our food delivery startup, recently. In the interest of avoiding redundancy, I won't rehash
            the same information as I've already provided above. Suffice it to say, it's been a great experience, hopefully with big things to come.`
          ]
        },
        {
          text: "Lab Tutor, UC Irvine, September 2021 - present",
          subitems: [
              `Working as an ICS lab tutor this fall, I've been supporting students in ICS 53 (Principles in System Design) by supervising open lab hours.
              On a daily basis, I help students understand course concepts & debug their C programs.`
          ]
        },
        {
          text: "Undergraduate Researcher, UC Irvine, September 2021 - December 2021",
          subitems: [
              `During Fall 2021, I worked on network databases & graph alignment with Prof. Wayne Hayes. My group was mainly concerned with
              developing ways to find good seeds for graph alignment, especially for applications in computational biology.`
          ]
        },
        {
          text: "Software Engineering Intern, Facebook, June 2021 - September 2021",
          subitems: [
            `At Facebook, I worked on an internal compiler toolchain, where I implemented runtime instrumentation for a new custom profiling system, and
            tooling around the new profiling system to assess & monitor profile health. Overall, I really enjoyed my summer and I'm excited to be back for summer 2022!`
          ]
        },
        {
          text: "Software Developer, UCI Student Center & Event Services, February 2020 - present",
          subitems: [
            `As a software engineer at the Student Center, I worked full-stack on iOS apps used by thousands of UCI students and staff on a daily basis (UCI students may
            recognize ZotFinder and UCI Now among them).`
          ]
        },
        {
          text: "Software Engineering Intern, Centric Software, June 2020 - September 2020",
          subitems: [
            `I spent summer 2020 as an intern with Centric's mobile development team, working with Swift & Core Graphics to create a canvas editor
            for Centric's suite of PLM iOS apps.`
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

h3 {
  font-size: 16pt;
  font-weight: 100;
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

#contact p {
  padding-left: 2vw;
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
