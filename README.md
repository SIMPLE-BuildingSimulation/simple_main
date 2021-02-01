SIMPLE - Simulating Buildings for People
=========================================

This is the main repository of the SIMPLE building simulation program. 


SIMPLE is an experimental Building Performance Simulation tool
developed with the purpose of more appropriately integrating
how "people" experience and interact with the buildings they use. 
Specifically, the research behind SIMPLE relates to 
residential environments. The requirements for such a tool 
were:

* __It had to be able to perform holistic simulations__: 
When people get into a room they just *feel* it. They do not 
separate—as Software and Building Scientists—the Thermal from 
the Daylight from the Acoustic domains. So, SIMPLE had to 
enable holistic Building Performance Simulation
* __It had to go beyond Building Physics__: For people, 
comfort is only part of their lives and thus they will, 
sometimes, tolerate "uncomfortable" situations in order to
satisfy other areas of their wellbeing. For instance, 
regardless of how hot their home is, people might choose not 
to open any window at night because of safety concerns, or 
simply because bugs can come in. These are things that SIMPLE 
had to be able to consider.
* __It had to be able to simulate imaginary futures__: Have 
you noticed that people tend to put a jacket on *before* going
out, instead of waiting until they actually feel cold? That 
is because, contrary to Occupants, People are always thinking 
ahead. People's behaviour—e.g., opening and closing 
windows, turning heaters on or off, putting more or less 
clothes—is often a reaction to what might happen later. This 
implies that SIMPLE had to be able to, somehow, infer the 
future and then come back to the present. 

If you want to understand a bit more the difference I make between Occupants and People [read this](https://buildingsforpeople.org/2020-08-14.blog).



Since adding these features into traditional tools required 
interventions that were way too big to be possible, SIMPLE 
was developed from scratch. __What you can find in the 
repositories now, however, is mostly a mock-up.__ That is to 
say—due to time and resource restrictions—the emphasis has 
been put on identifying the design patterns that would allow
accomplishing integrating the require features explained 
above. The physics, on its part, is just good enough to show 
us that the design pattern works and that the way People's 
experience in the building has been coded seems to be all 
right. Everyone is welcome to contribute to what is missing.Contact me for that (german.molinalarrain@vuw.ac.nz).


As you can see in the dependencies (Cargo.toml file), SIMPLE
has been designed with modularity in mind. Again, everyone is 
welcome to contribute.
