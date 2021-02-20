<template>
  <BlankSection name="intro">
    <div id="intro-content">
      <div id="bio">
        <h1 class="navy">Hi,</h1>
        <h3 class="navy">I'm Arthur Lafrance</h3>

        <p>Although I was born and raised in Silicon Valley, I currently study computer science
          at the UC Irvine Donald Bren School of ICS, as a member of the class of 2023. I'm very broadly interested in a wide
          range of subjects: artificial intelligence &amp; machine learning and software engineering of various kinds.

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

  <EntryList name="projects" title="Projects" :entries="projects" />
  <EntryList name="research" title="Research Experience" :entries="research" />
  <EntryList name="experience" title="Professional Experience" :entries="experience" />

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
        "This coming summer (summer 2021), I'll be working as a software engineering intern at Facebook.",
        "For Fall 2020 & Winter 2021, I serve as a learning assistant for ICS 31 (Intro to Programming).",
        "During summer 2020, I interned with Centric Software's iOS development team.",
        "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack."
      ],
      projects: [
        {
          title: "Pensiv",
          subtitles: [],
          description: `Pensiv is an artificial intelligence & machine learning library written in Rust that I'm currently in the early stages of
          developing. Two things happened recently that led to the creation of this project: I (finally) started pursuing my interest in AI/ML in both
          academics and free time, and I became interested in Rust. Outside of its compile-time safety guarantees & great type system, I gravitated toward
          Rust for its untapped potential for AI/ML -- it provides C-like performance with runtime-less memory safety, which is an unmatched combo as far as I've seen.
          While it does come at the cost of a steep learning curve, which I'm admittedly still grappling with, I've been eager to explore what's largely been
          uncharted (or at least not-as-charted) territory for Rust development. Anyway, on the subject of the actual project, at this point I'm basically
          playing catchup with a growing laundry list of AI/ML techniques that I'm meaning to implement. I'll also be doing lots of benchmarking and optimization
          when the baseline implementations are done. To date, I've completed basic probability/statistics functionality, and adversarial search is very nearly done.
          After that I'll be shifting my attention to foundational techniques for classification, regression/optimization, and clustering, at which point I'll branch out
          into further areas of AI/ML-related interest. Overall, Pensiv has so far been a fascinating combination of software engineering and mathematical/computational
          challenges, and I'm looking forward to continuing to expand it.`,
          link: "https://github.com/arthurlafrance/pensiv"
        },
        {
          title: "Tapestry",
          subtitles: [],
          description: `The context required to fully understand this project traces back to the experience two of my friends and I had in ICS 33, the third class
          in the UCI introductory CS series (but the second class we took for... reasons). Anyway, in the early parts of that class Professor Pattis (the one and only) used
          "ASCII memory diagrams" for visual demonstrations of the content we were learning. These diagrams were basically ASCII art-like representations of
          a program's state, so the Python variable assignment \`x = 1\` would have a "box" enclosing x, an "ellipse" enclosing 1, and an "arrow" linking them.
          We figured that these diagrams were very useful for students who wanted a visual representation of what was happening in their programs, so we created Tapestry
          at HackUCI 2020, which generates these "memory diagrams" dynamically during the runtime of arbitrary Python code. Granted, we used SVGs rather than ASCII art
          to render our diagrams, but we tried to infuse Tapestry with as much of Prof. Pattis's minimalist flavor as possible. After being fortunate enough to win Best
          Entrepreneurial Hack at HackUCI, we continued expanding Tapestry until it could, for the most part, generate an appropriate diagram for any arbitrary Python program,
          appropriately sandboxed of course. To date, Tapestry is probably the most complex piece of software I've built from scratch, so needless to say I've had a great time
          technically and personally working with my good friends on this project.`,
          link: "https://www.tapestrylearn.com"
        },
      ],
      research: [
        {
          title: 'Undergraduate Researcher, UC Irvine',
          subtitles: ['January 2021 - present', 'Adviser: Wayne Hayes'],
          description: `I'm currently conducting research under the direction of Prof. Wayne Hayes, concerned with the development of
          classification methods backed by statistical significance. My work mostly involves the implementation of a
          novel nearest neighbor distance classifier; I've worked on implementing result visualization, and I'm currently focusing on implementing
          optimized nearest neighbor search so that the classifier scales up better to large datasets. To do all this, I've pretty consistently used
          Python, Numpy, Matplotlib, and scikit-learn.`
        }
      ],
      experience: [
        {
          title: 'Software Engineering Intern, Centric Software',
          subtitles: ['June 2020 - September 2020'],
          description: `During summer 2020, I interned with Centric's mobile app development team, where I created a
            product mockup canvas editor for use in Centric's iOS apps. I architected, designed, and developed the editor,
            capable of standard shape creation/manipulation using Swift, and integrated it with
            Centric's existing infrastructure & apps using mainly Objective-C. I also became more comfortable working in a professional
            software engineering environment (particularly remotely), and thoroughly enjoyed my time working with the mobile team.`
        },
        {
          title: 'Software Developer, UCI Student Center & Event Services',
          subtitles: ['February 2020 - present'],
          description: `Throughout my time at UCI (beginning Winter Quarter 2020), I've worked with the Student Center,
            maintaining a set of several UCI-related iOS apps (UCI students may be particularly interested to hear that
            one such app is ZotFinder, the unofficial official choice for on-campus navigation). As part of this, I work mainly
            with Swift, Python (running on AWS Lambda), and various database technologies (AWS DynamoDB & EC2-based MySQL).
            Through working with the Student Center, I have honed my skills with full-stack iOS development,
            and also learned to better coexist with a team of cross-functional developers.`
        },
      ],
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

#intro-content p {
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
