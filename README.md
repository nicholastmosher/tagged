# Project

# 2026 Jan 28

I want to write down some guiding values and principles for this project, because
I want to pull this together from a mess of ideas into something actionable and useful.

## Project Vision and Values

Today, I see the centralization of software as a threat to Democracy. In the last
few decades, we have allowed our core productivity tools to shift from the Desktop
to the Cloud. At the time, the appeal was a profound increase in the ability to
collaborate, such as multi-cursor editing in google docs. However, it came at the
cost of data ownership, privacy, and soverignty, because the form-factor of Cloud
technology puts the data in control of the providers, not the users.

Consider the Browser, which is a marvel of technology. It is less of an application
and more of an application-platform. It's ultimately a program that can fetch content
from a network and render it to the screen. The content may be static, like a blog,
or dynamic, drawn by application code (JS/WASM) that itself is a form of content.
The Browser is the platform which provides the tools to draw on screen, make network
requests, play audio and video, and read/write to disk, and also takes on the responsibility
for isolating sites from one another to protect user privacy.

However, the greatest weakness of the Browser is that it fundamentally is only a **portal**
to view data that lives remotely on a server. When you use any web service, you're
not interacting with _your data on somebody else's machine_, you're interacting with
_somebody else's data **about you** on their machine_. Under this model, users are at
the mercy of application providers:

- Content that was once available to you may become unavailable
- Content that was once free may become paywalled
- Data that you consider private may be used by providers to train AI
- Data about you may be shared with arbitrary third-parties
- Providers may track your browsing and build a profile of you for purposes
  like targeted advertising or surveillance
- Your interactions with other users may be tracked and profiled
- In social media settings, the content you see is determined _by the provider_
  rather than by you, so people can be divided and isolated in bubbles.
- Content can be suppressed or boosted by providers, influencing public opinion
  and radicalizing users

The vision of this project is to create a next-generation application platform which
puts users in control of their data by using durable local-first and peer-to-peer
foundations. The spirit of this project is staunchly anti-capitalist, anti-fascist,
pro-democracy, and pro-sovereignty. Human beings should be able to use digital systems
while maintaining an expectation of privacy and anonymity, while also enjoying the
capabilities of live collaboration and group sharing.

# 2026 Jan 6

Status

- Working libp2p swarm, can detect peers with MDNS, good enough to start testing
- Need to get libp2p-stream connections established, unlocks e.g. automerge-repo

Todo

- Want to create mock discord-esque ui, where discord servers now correspond to Willow
  namespaces (or subspaces, yet to be seen)
- Add contacts window like discord friends, populate suggestions from detected peers?

# 2025 Dec 5

I've written about this elsewhere but I can't find my notes so I'm going to write a summary here.
I want Zed to become a pluggable ecosystem for p2p local-first applications. I've already got a
fork of Zed that allows plugins in the shape of `fn init(cx: &mut App)` that works.
I want to prototype at least one full vertical including a p2p network and a distributed file system.
However I eventually want to have abstractions over network type and file system. For example, here
are building blocks I've been eyeing for some time:

- Willow for data store / filesystem
- Iroh for p2p network (or libp2p, etc.)
- Automerge for collaborative editing (but I also now see iroh-docs)
- Zed itself has a native CRDT Text type, I'd want other doc formats to be pluggable so they could
  be used first-class in Zed buffers/editors.

One of the main challenges I foresee trying to abstract over several types of systems is needing some
way to unify an identity system. P2p systems tend to use some kind of public key for identity, but it's
not guaranteed for this to be the case and even so, there may be different key types. For example, Iroh
and Libp2p may have different formats used for public keys / Peer IDs. I think there may be a way to use
signed messages as a way to "associate" identities/keys, but I also worry about leaking such associations
in a world where privacy/anonymity should be treated as first-class.

For now, I think I'll choose one full vertical stack and try to make an end-to-end prototype. Today I'm
mostly interested in Willow, Iroh, and Automerge. I see Iroh already has a working example of an automerge
repository. I think that Willow could serve as more of a "cold store" which is also the basis for permissioning,
given that Willow+Meadowcap already have permissioning and privacy built in mind. I think Automerge documents
could be cold-saved into Willow and be subject to Namespace/Subspace rules and capability sharing. On opening
a document, we could create a "Session" which is essentially loading into automerge-repo/samod to do live
editing. We could write a Session file into Willow, then when it syncs to other Willow peers, the hosting
Zed plugin could visually show that the document is being edited by somebody (subject to the visibility rules
of Willow namespaces).

Side thought: Could a standard OS filesystem be expressed as a special case of a Willow namespace? In other
words, could Willow's own API be used as a filesystem abstraction that Zed could be pointed to, so that Zed
could be a first-class editor over both OS and Willow filesystems? Similarly, could we find an abstraction
that encompasses both Automerge and Zed CRDTs?

Today's plan (because I need to focus)

- Follow the iroh-examples/iroh-automerge-repo example and try to integrate it visually in Zed
- Can start with ad-hoc Iroh keys but eventually need a key store / identity/contact management system
