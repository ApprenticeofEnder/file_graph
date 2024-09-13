import ForceGraph2D from "react-force-graph-2d";
import { NodeObject, LinkObject } from "react-force-graph-2d";

function genRandomTree(N = 10, reverse = false) {
  return {
    nodes: [...Array(N).keys()].map((i) => ({ id: i, filename: "test" })),
    links: [...Array(N).keys()]
      .filter((id) => id)
      .map((id) => ({
        [reverse ? "target" : "source"]: id,
        [reverse ? "source" : "target"]: Math.round(Math.random() * (id - 1)),
      })),
  };
}

function App() {
  return (
    <>
      <h1 className="text-foreground text-4xl">Hello</h1>
      <ForceGraph2D
        graphData={genRandomTree()}
        nodeCanvasObject={(data) => {
          console.log(data);
        }}
      />
    </>
  );
}

export default App;
