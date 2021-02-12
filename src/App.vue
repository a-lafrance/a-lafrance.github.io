<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi,</h1>
        <h3 class="navy">I'm Arthur Lafrance</h3>

        <p>Although I was born and raised in Silicon Valley, I currently study computer science
          at the UC Irvine Donald Bren School of ICS, as a member of the class of 2023. I'm very broadly interested in a wide
          range of subjects: artificial intelligence &amp; machine learning, programming languages/operating systems/distributed systems,
          and software engineering.

          <br /><br />

          Whether through personal projects, professional experience, or academics, I'm always furthering these interests -- in my free time, I can almost
          always be found coding, reading, &amp; learning. My personal favorite thing I'm learning right now is the Rust programming language; although the learning curve is steep,
          its compile-time memory &amp; concurrency safety guarantees are fascinating.
        </p>
      </div>

      <div id="contact">
        <p>To learn more about me or get in touch, feel free to check out:</p>

        <div class="button-group">
          <Button class="navy" v-for="(method, index) in contactMethods" :key="index" :title="method.name" :destination="method.link" />
        </div>
      </div>
    </div>
  </BlankSection>

  <Section name="highlights" title="Highlights">
    <List :items="highlights" />
  </Section>

  <EntryList name="research" title="Research Experience" :entries="research" />
  <EntryList name="experience" title="Professional Experience" :entries="experience" />
  <EntryList name="teaching" title="Teaching Experience" :entries="teaching" />
  <EntryList name="projects" title="Projects" :entries="projects" />

  <div class="footer">
    <p class="copyright navy">Copyright (c) 2021 Arthur Lafrance</p>
  </div>
</template>

<script>
import List from './components/List.vue'
// import Link from './components/Link.vue'
import Button from './components/Button.vue'
// import Entry from './components/Entry.vue'
import BlankSection from './components/BlankSection.vue'
import Section from './components/Section.vue'
import EntryList from './components/EntryList.vue'

export default {
  name: 'App',
  components: {
    List,
    // Link,
    Button,
    // Entry,
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
        "This coming summer (summer 2021), I'll be working as a software engineering intern at Facebook.",
        "For Fall 2020 & Winter 2021, I serve as a learning assistant for ICS 31 (Intro to Programming).",
        "During summer 2020, I interned with Centric Software's iOS development team.",
        "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack."
      ],
      research: [
        {
          title: 'Undergraduate Researcher, UC Irvine',
          subtitles: ['January 2021 - present', 'Adviser: Wayne Hayes'],
          description: `Beginning in winter 2021, I've begun research under the direction of Prof. Wayne Hayes. The
            core problem the group is currently solving is developing classification methods backed by statistical significance.
            My work is mostly concerned with the implementation of a novel nearest neighbor distance classifier, implementing components
            of the classifier such as nearest neighbor search, testing infrastructure such as results plotting, and optimizing the classifier to
            scale up well to large datasets.`
        }
      ],
      experience: [
        {
          title: 'Software Engineering Intern, Centric Software',
          subtitles: ['June 2020 - September 2020'],
          description: `During summer 2020, I interned with Centric's mobile app development team, where I created a
            product mockup canvas editor for use in Centric's iOS apps. I architected, designed, and developed the editor,
            capable of basic shape creation and manipulation (both graphical and physical) using Swift, and integrated it with
            Centric's existing infrastructure using mainly Objective-C. I also became more comfortable working in a professional
            software engineering environment (particularly remotely), and thoroughly enjoyed my time working with the mobile team.`
        },
        {
          title: 'Software Developer, UCI Student Center & Event Services',
          subtitles: ['February 2020 - present'],
          description: `Throughout my time at UCI (beginning Winter Quarter 2020), I have worked with the Student Center,
            maintaining a set of several popular UCI-related iOS apps (UCI students may be particularly interested to hear that
            one such app is ZotFinder, the unofficial official choice for on-campus navigation). As part of this, I work mainly
            with Swift, Python (using Chalice and AWS Lambda), and various database technologies (AWS DynamoDB & MySQL running on
            AWS EC2). Through working with the Student Center, I have honed my skills with backend & full-stack iOS development,
            and also learned to better coexist with a team of like-mindedly passionate developers.`
        },
      ],
      teaching: [
        {
          title: 'Learning Assistant - ICS 31, UC Irvine',
          subtitles: ['Fall 2020 & Winter 2021'],
          description: `For Fall 2020 & Winter 2021, I serve as a learning assistant for ICS 31 (Intro to Programming). As part
            of this, I hold office hours for several hours per week and manage a small
            group during lecture, where students collaborate to solve participation activities and apply the concepts they learn, all
            with the goal of facilitating student learning and guiding them toward mastery of the material.`
        }
      ],
      projects: [
        {
          title: "Pensiv",
          subtitles: [],
          description: `Recently, I've become interested in Rust for its compile-time memory safety, ergonomic syntax,
            and high performance. This has coincided with my (continuing) introduction to AI and machine learning, so I reasoned that
            a great way to combine the two would be to create Pensiv,
            a Rust library (crate) providing my own implementations of various AI & machine learning techniques. To date, I've implemented basic probability & statistics, and adversarial search is actively
            in development; I have no doubt that this project will continue to grow as I become more experienced and thus have more
            knowledge and abilities to share through the crate. In the near future, I'm hoping to expand to basic classification,
            regression, and clustering techniques, and I'll probably toy with the implementation of neural networks and graphical models
            in the future as well. Overall, I'm grateful to have the opportunity to further my passions for Rust & machine learning through
            continued development of Pensiv.`,
          link: "https://github.com/arthurlafrance/pensiv"
        },
        {
          title: "Tapestry",
          subtitles: [],
          description: `At HackUCI 2020, two good friends and I created Tapestry, a web-based
            code visualization tool inspired by the "ASCII memory diagrams" used by one of our professors (which we were
            fortunate enough to be awarded Best Entrepreneurial Hack for). We aimed to create a tool that could generate these
            diagrams for arbitrary Python programs, in order to provide an interactive visual learning experience for computer
            science students. Since the hackathon, we have expanded our initial proof-of-concept to more fully realize this goal,
            creating a frontend website using TypeScript and Vue that provides a minimalist IDE and diagram display, and a
            backend API using Python, Django, and SQLite that remotely executes & generates diagrams for Python code snippets.`,
          link: "https://www.tapestrylearn.com"
        },
        {
          title: "Trivia Trek",
          subtitles: [],
          description: `For the 2019 FBLA Mobile App Development competition, I worked with 2 other developers to create
            Trivia Trek, and iOS trivia board game to help users learn about FBLA trivia. We created the game using Swift,
            SpriteKit, and Firebase, where a user answers a series of questions in order to progress along one of two "maps"
            as fast as possible. Particularly, I worked on the app's frontend (gameplay) & backend (integration with Firebase),
            and was also in charge of maintaining the backend infrastructure (i.e. Firebase database). For our work on Trivia Trek, my team was
            fortunate enough to receive recognition at both the state & national levels of competition, placing 2nd at the CA
            FBLA State Leadership Conference and in the Top 15 at the FBLA National Leadership Conference.`
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
  color: rgb(50, 50, 50);
}

h3 {
  font-size: 16pt;
  font-weight: 100;
  color: rgb(50, 50, 50);
}

.button-group {
  display: flex;
  flex-direction: column;

  padding-left: 3vw;
  padding-right: 0;
}

#intro-content {
  display: flex;
}

#bio {
  width: 50%;
  padding-right: 1vw;
}

#intro p {
  font-size: 12pt;
}

#contact {
  padding-left: 1vw;
  padding-top: 4%;
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
