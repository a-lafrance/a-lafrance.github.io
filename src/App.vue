<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi,</h1>
        <h3 class="navy">I'm Arthur Lafrance</h3>

        <div id="bottom">
          <p>Born &amp; raised in Silicon Valley; currently studying computer science at UC Irvine, class of 2023. Also
             in between summers at Facebook. I'm broadly interested in compilers, programming language design, &amp; computer
             architecture.
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
    <p class="copyright navy">Copyright (c) 2021 Arthur Lafrance</p>
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
        { name: "CV", link: "assets/Arthur_Lafrance_CV.pdf" },
      ],
      highlights: [
        { text: "I'll be returning to Facebook for summer 2022, likely (hopefully) to continue working with PLs & compilers.", subitems: [] },
        { text: "I spent summer 2021 at Facebook, where I worked on compilers, profiling, & infrastructure.", subitems: [] },
        { text: "During summer 2020, I interned with Centric Software's iOS development team.", subitems: [] },
        { text: "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack.", subitems: [] }
      ],
      currentWork: [
        {
            title: "lfc",
            subtitles: [],
            description: `To be brutally honest, C doesn't give you much out of the box. That's part of the reason I enjoy using it, but I can already
            foresee that it'll be annoying to write the same set of utilities 50 different times per-project. So, lfc is my personal solution to that problem, point
            being to basically fill any niche I need that the C (and POSIX I guess) standard library doesn't adequately provide. Some days it's really cool, and
            others it feels like I'm writing the code equivalent of bitter medicine, where I just have to finish it because I know it's good for me.`
        },
        {
            title: "Mustang",
            subtitles: [],
            description: `Recently I've been experimenting with the idea of writing a C toolchain (in C of course). I figure it'll probably be useful when I
            start writing more and more C projects, because who wants to set up the same Makefile and directory structure and manually build everything all the time. Instead,
            it seems more fun to write the stuff I need myself, much the same as with lfc. Who knows whether or not it'll pan out or turn out as useful as it seems, but it's
            worth a shot.`
        },
        {
          title: "???",
          subtitles: [],
          description: `Check back later after the big reveal 👀`
        }
      ],
      pastWork: [
        {
          title: "Tapestry",
          subtitles: [],
          description: `The context to fully appreciate this project goes back to ICS 33 (the last part of our UCI intro CS series), when Prof. Pattis (the one and only) created ASCII art diagrams
          to visually demonstrate how Python objects are organized in memory. Later, my two good friends & I were searching for project ideas at HackUCI 2020
          when we decided to create a website that could dynamically generate these "Pattis diagrams" for arbitrary Python code. We got a decent proof-of-concept
          going at the hackathon, for which we were fortunate enough to win Best Entrepreneurial Hack, and we then spent the next 6 months or so fully bringing
          the idea to life. We went through several full rewrites of the app, learned a ton about Python, Django, and software design, and eventually did end up with
          a functional website that did what we originally intended. To date I think it's the most complex piece of software I've built from scratch, so needless to say
          I had a great technical and personal experience building it with my team.`, // TODO: insert links to ryan/patrick sites
          link: "https://www.github.com/tapestrylearn/Diagrammer"
        }
      ],
      resumeDump: [
        {
            text: "Lab Tutor, UC Irvine, September 2021 - present",
            subitems: [
                `Working as an ICS lab tutor this fall, I've been supporting students in ICS 53 (Principles in System Design) by supervising open lab hours.
                On a daily basis, I help students understand course concepts, debug their systems-level C programs, & gain proficiency with essential Unix tools like GDB.`
            ]
        },
        {
            text: "Undergraduate Researcher, UC Irvine, September 2021 - present",
            subitems: [
                `During Fall 2021, I've been working on network databases & graph alignment with Prof. Wayne Hayes. My group is mainly concerned with
                developing ways to find good seeds for graph alignment, mainly useful for applications in computational biology.`
            ]
        },
        {
          text: "Software Engineering Intern, Facebook, June 2021 - September 2021",
          subitems: [
            `At Facebook, I worked on an internal compiler toolchain, where I implemented runtime instrumentation for a new custom profiling system, which
            was found to produce binaries that were ~60% smaller & that carried ~95% less instrumentation data. To support toolchain users, I extended existing
            tooling surrounding the profiling system and created new tools as well, both with the goal of assessing profile health. Overall, I really enjoyed my summer
            and I'm excited to be back for summer 2022!`
          ]
        },
        {
          text: "Software Developer, UCI Student Center & Event Services, February 2020 - present",
          subitems: [
            `At the Student Center I've gotten a chance to work on some pretty cool apps (UCI students might recognize ZotFinder & UCI Now among them) and
            grow significantly as a developer. As a full-stack iOS developer, I've worked with Swift, Python, AWS, & MySQL. I've really enjoyed getting the
            chance to make an impact on the campus community through my work.`
          ]
        },
        {
          text: "Undergraduate Researcher, UC Irvine, January 2021 - April 2021",
          subitems: [
            `In winter 2021, I spent the quarter with Prof. Wayne Hayes's group investigating machine learning methods with statistically-backed results.
            I did some initial prototyping & investigated fast nearest neighbor search methods to scale up the classifier; I researched, prototyped, & benchmarked
            several tree-based methods (k-d and ball trees).`
          ]
        },
        {
          text: "ICS 31 Learning Assistant, UC Irvine, Fall 2020 & Winter 2021",
          subitems: [
            `As a learning assistant for ICS 31, I spent 2 quarters helping students in UCI's intro to programming class (in Python) by holding office hours
            several times a week and by facilitating breakout rooms during lecture, answering questions about lecture material & assignments.`
          ]
        },
        {
          text: "Software Engineering Intern, Centric Software, June 2020 - September 2020",
          subitems: [
            `I spent summer 2020 as an intern with Centric's mobile development team. Specifically, I developed a canvas editor for their suite of
            PLM apps on iOS, creating a basic working canvas editor from the ground up using Swift and Core Graphics, and I also spent some time later on
            integrating it into the Sample Review app using Objective-C. Overall, it was a great learning experience taking a set of
            system requirements, architecting the system from scratch, and developing the system from start to finish.`
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

  padding-left: 0.5vw;
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
</style>
