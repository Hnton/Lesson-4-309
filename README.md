# Lesson-4-309

Create a data structure to represent a memory page (see page 391).  

Write an application that will

Generate a reference string of 100 random page numbers between 0 and 19
Have a frame buffer (an array of pages representing main memory)
Test the reference string/frame buffer with each of the following algorithms
FIFO
NRU
Second Chance
Clock
LRU
  Using the same 100 element randomly generated reference string, test each algorithm
Use a frame buffer size of 3, 5, and 10 (three tests for the same reference string and each algorithm). Be sure to "empty" the buffer each time a test is done.
  For each test, report on the number of page faults and hits (reads of in memory pages), and the number of pages removed from memory
Report on which algorithm is the best and worst under each condition
