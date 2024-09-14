import React, { useEffect, useRef } from "react";

import RelationGraph from "relation-graph-react";

function genRandomTree(N = 10, reverse = false) {
  return {
    rootId: "0",
    nodes: [...Array(N).keys()].map((i) => ({ id: `${i}`, filename: "test" })),
    links: [...Array(N).keys()]
      .filter((id) => id)
      .map((id) => ({
        [reverse ? "to" : "from"]: `${id}`,
        [reverse ? "from" : "to"]: `${Math.round(Math.random() * (id - 1))}`,
      })),
  };
}

function App() {
  return (
    <>
      <h1 className="text-foreground text-4xl">Hello</h1>
    </>
  );
}

export default App;
