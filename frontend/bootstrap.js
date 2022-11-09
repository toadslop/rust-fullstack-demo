import "./styles.scss";

import("./pkg/index.js").then((module) => {
  module.main();
});
