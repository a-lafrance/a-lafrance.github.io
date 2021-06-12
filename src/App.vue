<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi,</h1>
        <h3 class="navy">I'm Arthur Lafrance</h3>

        <div id="bottom">
          <p>Although I was born and raised in Silicon Valley, I currently study computer science
            at the UC Irvine Donald Bren School of ICS, as a member of the class of 2023. I'm very broadly interested in
            computer architecture &amp; systems, programming languages, and AI/machine learning.
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

<!--
  <EntryList name="current-work" title="Things I'm Working On" :entries="currentWork">
    <p class="description">A short portfolio of cool stuff I'm currently building</p>
  </EntryList>

  <EntryList name="past-work" title="Things I've Worked On" :entries="pastWork">
    <p class="description">Some interesting projects I worked on in the past</p>
  </EntryList>
-->
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
// import EntryList from './components/EntryList.vue'

export default {
  name: 'App',
  components: {
    List,
    Button,
    BlankSection,
    Section//,
    // EntryList
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
        { text: "This coming summer (summer 2021), I'll be working as a software engineering intern on the Pika team at Facebook.", subitems: [] },
        { text: "During summer 2020, I interned with Centric Software's iOS development team.", subitems: [] },
        { text: "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack.", subitems: [] }
      ],
      resumeDump: [
        {
          text: "Software Developer, Blueprint at UCI, February 2021 - present",
          subitems: [
            `As a developer at Blueprint, I've been building a volunteer portal for Our City Forest, a Bay Area sustainability nonprofit,
            allowing them to coordinate and manage volunteer signups online. I've designed & implemented the relational DB using MySQL and
            turned mockups into code using React & Express.`
          ]
        },
        {
          text: "Software Developer, UCI Student Center & Event Services, February 2020 - present",
          subitems: [
            `At the Student Center I've gotten a chance to work on some pretty cool apps (UCI students might recognize ZotFinder & UCI Now among them) and
            grow significantly as a developer. I was originally hired as an iOS developer, working full-stack with Swift & Python/AWS; recently I've been
            transitioning into a web development role as well, so I've been picking up C# in addition to my work on iOS.`
          ]
        },
        {
          text: "Undergraduate Researcher, UC Irvine, January 2021 - April 2021",
          subitems: [
            `In winter 2021, I spent the quarter with Prof. Wayne Hayes's group investigating machine learning methods with statistically-backed results.
            I did some initial prototyping & investigated fast nearest neighbor search methods to scale up the classifier; I researched & prototyped
            several tree-based methods (k-d and ball trees) & benchmarked them relative to linear search at large scales,
            all using Numpy, Matplotlib, and scikit-learn.`
          ]
        },
        {
          text: "ICS 31 Learning Assistant, UC Irvine, Fall 2020 & Winter 2021",
          subitems: [
            `As a learning assistant for ICS 31, I spent 2 quarters helping students in UCI's intro to programming class by holding office hours
            several times a week and by facilitating breakout rooms during lecture.`
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
