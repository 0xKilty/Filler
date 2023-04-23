import "./App.css";
import { Box } from "@mui/system";
import {
  ThemeProvider,
  createTheme,
  Typography,
  Button,
  Grid,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  DialogContentText,
} from "@mui/material";
import ColorPicker from "./ColorPicker";
import { useEffect, useState } from "react";
import Typewriter from "typewriter-effect";
import FiberManualRecordIcon from "@mui/icons-material/FiberManualRecord";
import EmojiEventsSharpIcon from "@mui/icons-material/EmojiEventsSharp";
import CloseSharpIcon from "@mui/icons-material/CloseSharp";
import AllInclusiveSharpIcon from "@mui/icons-material/AllInclusiveSharp";
import init, { algo_move, bfs } from "./pkg/hello_wasm";

var ghost = init();

function App() {
  const theme = createTheme({
    palette: {
      primary: {
        main: "#252524",
        contrastText: "#ffffff",
      },
      secondary: {
        main: "#fff",
        red: "#e45968",
        yellow: "#fae253",
        green: "#b4d975",
        blue: "#67b0f0",
        purple: "#6b53a0",
        black: "#464747",
        contrastText: "#000000",
      },
    },
    typography: {
      header: {
        fontSize: 48,
        fontWeight: 700,
      },
      body: {
        fontSize: 20,
        fontWeight: 700,
      },
      typed: {
        fontSize: 16,
        fontWeight: 700,
      },
      subHeader: {
        fontWeight: 700,
        fontSize: 28,
      },
    },
  });

  const buttonStyle = (color) => {
    return {
      color: "primary.contrastText",
      textTransform: "none",
      marginTop: "1rem",
      marginLeft: "1rem",
      "&:hover": {
        backgroundColor: color,
        color: "primary.main",
      },
    };
  };

  const colors = [
    "#e45968",
    "#fae253",
    "#b4d975",
    "#67b0f0",
    "#6b53a0",
    "#464747",
  ];

  const [ownership, setOwnership] = useState([
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 4, 3, 5, 5, 0, 0, 0, 0,
    0, 0, 0, 4, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5,
    5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 2, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1, 2, 0, 0,
    0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
  ]);

  const [board, setBoard] = useState([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 4, 4, 5, 6, 1, 3, 0, 0, 1, 2, 4, 4,
    5, 6, 1, 2, 0, 0, 1, 2, 4, 4, 5, 6, 1, 2, 0, 0, 1, 2, 4, 4, 5, 6, 1, 2, 0,
    0, 1, 2, 4, 4, 5, 6, 1, 2, 0, 0, 1, 2, 4, 4, 5, 6, 1, 2, 0, 0, 2, 2, 4, 4,
    5, 6, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ]);

  const boxStyle = (color) => {
    return {
      width: 64,
      height: 64,
      display: "inline-block",
      backgroundColor: color,
    };
  };

  const [scores, setScores] = useState([1, 0, 1]);
  const [blockedColors, setBlockedColors] = useState([]);
  const [update, setUpdate] = useState(false);

  const newGame = () => {
    var tempBoard = board;
    for (var i = 11; i < 79; i++) {
      if (i % 10 !== 0 && (i + 1) % 10 !== 0) {
        tempBoard[i] = Math.floor(Math.random() * 6) + 1;
        while (
          tempBoard[i] === tempBoard[i - 1] ||
          tempBoard[i] === tempBoard[i - 10]
        ) {
          tempBoard[i] = Math.floor(Math.random() * 6) + 1;
        }
      }
    }
    while (
      tempBoard[71] === tempBoard[18] ||
      tempBoard[71] === tempBoard[72] ||
      tempBoard[71] === tempBoard[61]
    ) {
      tempBoard[71] = Math.floor(Math.random() * 6) + 1;
    }
    setBlockedColors([tempBoard[71] - 1, tempBoard[18] - 1]);
    setBoard(tempBoard);
    window.localStorage.board = tempBoard;
    setScores([1, 0, 1]);
    window.localStorage.scores = [1, 0, 1];
    var tempOwnership = [
      5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 4, 3, 5, 5, 0, 0, 0, 0,
      0, 0, 0, 4, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5,
      5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 2, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1, 2, 0, 0,
      0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ];
    setOwnership(tempOwnership);
    window.localStorage.ownership = tempOwnership;
    setTurn(true);
    setWon([false, 1]);
    setUpdate(!update);
    setTouchIndexes([
      [17, 28],
      [61, 72],
    ]);
    setOwnerIndexes([[18], [71]]);
    setCaptured([]);
    if (window.localStorage.twoPlayer) {
      if (window.localStorage.twoPlayer == "true") {
        setTwoPlayer(true);
      } else {
        setTwoPlayer(false);
      }
    }
    if (!twoPlayer) {
      setTimeout(() => {
        setPhilFace("(¬‿¬)");
      }, 1);
      philSay("Hi, I'm Phil");
    }
  };

  const [turn, setTurn] = useState(true);
  const [won, setWon] = useState([false, 1]);
  const win = () => {
    if (scores[0] > scores[2]) {
      setWon([true, 1]);
      setTimeout(() => {
        setPhilFace("(¬︵¬)");
      }, 1);
      philSay("Filler is luck");
    } else if (scores[2] > scores[0]) {
      setWon([true, 2]);
      setTimeout(() => {
        setPhilFace("ᕦ(ò_óˇ)ᕤ");
      }, 1);
      philSay("Phil has filled");
    } else {
      setWon([true, 3]);
      setTimeout(() => {
        setPhilFace("(~_^)");
      }, 1);
      philSay("Tie?");
    }
  };

  const [touchIndexes, setTouchIndexes] = useState([
    [17, 28],
    [61, 72],
  ]);
  const [algoMove, setAlgoMove] = useState(true);
  const [ownerIndexes, setOwnerIndexes] = useState([[18], [71]]);
  const [captured, setCaptured] = useState([]);
  const [twoPlayer, setTwoPlayer] = useState(true);

  const move = (color) => {
    var tempBoard = board;
    var tempOwnership = ownership;
    var tempTouchIndexesRemove = [];
    var touchNum = 2;
    var oppTouchNum = 4;
    var indexNum = 1;
    var indexIndex = 1;
    color += 1;
    if (!turn) {
      touchNum = 4;
      indexIndex = 0;
      indexNum = 3;
      oppTouchNum = 2;
      tempTouchIndexes = touchIndexes[0];
    }
    var tempTouchIndexes = touchIndexes[indexIndex];
    var tempOwnerIndexes = ownerIndexes[indexIndex];
    var tempCaptured = captured;
    var score = 0;
    for (var i = 0; i < tempTouchIndexes.length; i++) {
      if (board[tempTouchIndexes[i]] === color) {
        tempOwnership[tempTouchIndexes[i]] = indexNum;
        var search = tempTouchIndexes[i];
        tempOwnerIndexes.push(search);
        tempTouchIndexesRemove.push(search);
        if (ghost !== "undefined") {
          if (
            search <= 18 &&
            ownership[search + 1] != indexNum &&
            ownership[search - 1] != indexNum
          ) {
            var bfs_res = bfs(tempOwnership, search + 1);
            bfs_res.forEach((element) => tempCaptured.push(element));
            bfs_res = bfs(tempOwnership, search - 1);
            bfs_res.forEach((element) => tempCaptured.push(element));
          } else if (
            search >= 71 &&
            ownership[search + 1] != indexNum &&
            ownership[search - 1] != indexNum
          ) {
            var bfs_res = bfs(tempOwnership, search + 1);
            bfs_res.forEach((element) => tempCaptured.push(element));
            bfs_res = bfs(tempOwnership, search - 1);
            bfs_res.forEach((element) => tempCaptured.push(element));
          } else if (
            search % 10 == 1 &&
            ownership[search + 10] != indexNum &&
            ownership[search - 10] != indexNum
          ) {
            var bfs_res = bfs(tempOwnership, search + 10);
            bfs_res.forEach((element) => tempCaptured.push(element));
            bfs_res = bfs(tempOwnership, search - 10);
            bfs_res.forEach((element) => tempCaptured.push(element));
          } else if (
            search % 10 == 8 &&
            ownership[search + 10] != indexNum &&
            ownership[search - 10] != indexNum
          ) {
            var bfs_res = bfs(tempOwnership, search + 10);
            bfs_res.forEach((element) => tempCaptured.push(element));
            bfs_res = bfs(tempOwnership, search - 10);
            bfs_res.forEach((element) => tempCaptured.push(element));
          }
        }
        if (tempOwnership[search - 1] === 0) {
          tempOwnership[search - 1] = touchNum;
          tempTouchIndexes.push(search - 1);
        } else if (tempOwnership[search - 1] === oppTouchNum) {
          tempOwnership[search - 1] = 6;
          tempTouchIndexes.push(search - 1);
        }
        if (tempOwnership[search + 1] === 0) {
          tempOwnership[search + 1] = touchNum;
          tempTouchIndexes.push(search + 1);
        } else if (tempOwnership[search + 1] === oppTouchNum) {
          tempOwnership[search + 1] = 6;
          tempTouchIndexes.push(search + 1);
        }
        if (tempOwnership[search - 10] === 0) {
          tempOwnership[search - 10] = touchNum;
          tempTouchIndexes.push(search - 10);
        } else if (tempOwnership[search - 10] === oppTouchNum) {
          tempOwnership[search - 10] = 6;
          tempTouchIndexes.push(search - 10);
        }
        if (tempOwnership[search + 10] === 0) {
          tempOwnership[search + 10] = touchNum;
          tempTouchIndexes.push(search + 10);
        } else if (tempOwnership[search + 10] === oppTouchNum) {
          tempOwnership[search + 10] = 6;
          tempTouchIndexes.push(search + 10);
        }
      }
    }
    for (var i = 11; i < 79; i++) {
      if (tempOwnership[i] === indexNum) {
        tempBoard[i] = color;
        score += 1;
      }
    }
    for (var i = 0; i < tempTouchIndexesRemove.length; i++) {
      tempTouchIndexes.splice(
        tempTouchIndexes.indexOf(tempTouchIndexesRemove[i]),
        1
      );
    }

    var tempIndexes = touchIndexes;
    tempIndexes[indexIndex] = tempTouchIndexes;
    setTouchIndexes(tempIndexes);
    window.localStorage.touchIndexes = tempIndexes;
    tempIndexes = ownerIndexes;
    tempIndexes[indexIndex] = tempOwnerIndexes;
    setOwnerIndexes(tempIndexes);
    window.localStorage.ownerIndexes = tempIndexes;
    setCaptured(tempCaptured);
    window.localStorage.captured = tempCaptured;
    var tempScores = scores;
    tempScores[indexNum - 1] = score;
    setScores(tempScores);
    window.localStorage.scores = tempScores;
    setBlockedColors([board[18] - 1, board[71] - 1]);
    setOwnership(tempOwnership);
    window.localStorage.ownership = tempOwnership;
    setBoard(tempBoard);
    window.localStorage.board = tempBoard;
    setTurn(!turn);
    setAlgoMove(!algoMove);
    window.localStorage.turn = turn;
    if (scores[0] + scores[2] === 56) {
      win();
    }
    setUpdate(!update);
  };

  var [topTypewriter, setTopTypewriter] = useState(null);
  const [philFace, setPhilFace] = useState("(¬‿¬)");
  const philSay = (something) => {
    topTypewriter.typeString(something).start().pauseFor(2000).deleteAll();
  };

  const handlePlayerNum = () => {
    setTwoPlayer(!twoPlayer);
    window.localStorage.twoPlayer = !twoPlayer;
    newGame();
  };

  const textScore = (color) => {
    if (color >= 4) {
      return "#ffffff";
    }
    return "#000000";
  };

  useEffect(() => {
    if (
      !window.localStorage.board ||
      !window.localStorage.ownership ||
      !window.localStorage.turn ||
      !window.localStorage.scores ||
      !window.localStorage.blockedColors ||
      !window.localStorage.twoPlayer ||
      !window.localStorage.touchIndexes ||
      !window.localStorage.ownerIndexes
    ) {
      newGame();
    } else {
      var tempBoard = window.localStorage.board.split(",").map(function (x) {
        return parseInt(x, 10);
      });
      setBoard(tempBoard);
      var tempOwnership = window.localStorage.ownership
        .split(",")
        .map(function (x) {
          return parseInt(x, 10);
        });
      setOwnership(tempOwnership);
      setTurn(window.localStorage.turn);
      var tempScores = window.localStorage.scores.split(",").map(function (x) {
        return parseInt(x, 10);
      });
      setScores(tempScores);
      setBlockedColors([tempBoard[18] - 1, tempBoard[71] - 1]);
      if (window.localStorage.twoPlayer == "true") {
        setTwoPlayer(true);
      } else {
        setTwoPlayer(false);
      }
      setTouchIndexes(window.localStorage.touchIndexes);
      setOwnerIndexes(window.localStorage.ownerIndexes);
    }
  }, []);

  useEffect(() => {
    if (!twoPlayer && !turn && !won[0]) {
      move(
        algo_move(
          board,
          ownership,
          touchIndexes[0],
          ownerIndexes[0],
          captured.length + ownerIndexes.length,
          captured
        )
      );
      if (Math.floor(Math.random() * 10) == 1) {
        if (scores[0] > scores[2]) {
          setTimeout(() => {
            setPhilFace("(¬_¬)");
          }, 1);
          philSay("You are winning.");
        } else if (scores[0] + 1 < scores[2]) {
          setTimeout(() => {
            setPhilFace("(¬‿¬)");
          }, 1);
          philSay("I am winning!");
        }
      }
      if (Math.floor(Math.random() * 10) == 1) {
        var quotes = [
          "Filler is fun",
          "I like red",
          "Hmmmm",
          "I'm colorblind",
          "Philler",
        ];
        var faces = ["(~‿~)", "(~_^)", "(ง'̀-'́)ง"];
        setTimeout(() => {
          setPhilFace(faces[Math.floor(Math.random() * 3)]);
        }, 1);
        philSay(quotes[Math.floor(Math.random() * 5)]);
      }
    }
  }, [algoMove]);

  useEffect(() => {}, [philFace]);

  useEffect(() => {}, [update]);

  const [open, setOpen] = useState(false);
  const handleAboutOpen = () => {
    setOpen(true);
  };

  const handleAboutClose = () => {
    setOpen(false);
  };

  useEffect(() => {}, [open]);

  const handleAbout = () => {
    window.open("https://0xkilty.github.io/#/Projects/Filler", "_blank");
  };
  return (
    <ThemeProvider theme={theme}>
      <Box
        sx={{
          width: "50%",
          minWidth: "44rem",
          margin: "auto",
          backgroundColor: "primary.main",
          }}
      >
        <Box
          sx={{
            width: "90%",
            margin: "auto",
            paddingTop: "5vh",
          }}
        >
          <Typography
            variant="header"
            color="primary.contrastText"
            sx={{ paddingLeft: "1rem" }}
          >
            Filler
          </Typography>
          <Box
            sx={{
              float: "right",
              display: "block",
            }}
          >
            <Button sx={() => buttonStyle(colors[0])} onClick={handleAbout}>
              <Typography variant="body">About</Typography>
            </Button>
            <Dialog
              open={open}
              onClose={handleAboutClose}
              PaperProps={{
                style: {
                  backgroundColor: "primary.main",
                  color: "primary.main",
                },
              }}
              sx={{
                margin: "1rem",
              }}
            >
              <DialogTitle sx={{ backgroundColor: "primary.main" }}>
                <Typography variant="header" color="primary.contrastText">
                  How to play
                </Typography>
              </DialogTitle>
              <DialogContent sx={{ backgroundColor: "primary.main" }}>
                <DialogContentText>
                  <Typography variant="typed" color="primary.contrastText">
                    Each player starts out with one color in each of the
                    corners. Press a color from the squares to become that color
                    and capture the squares of the same color touching it. The
                    game is over when there are no more squares to capture.
                  </Typography>
                </DialogContentText>
              </DialogContent>
              <DialogActions sx={{ backgroundColor: "primary.main" }}>
                <Button
                  onClick={handleAboutClose}
                  sx={{
                    color: "primary.contrastText",
                    textTransform: "none",
                    marginBottom: "1rem",
                    display:"block",
                    "&:hover": {
                      backgroundColor: "secondary.green",
                      color: "primary.main",
                    },
                  }}
                >
                  <Typography variant="body">Play</Typography>
                </Button>
              </DialogActions>
            </Dialog>
            <Button sx={() => buttonStyle(colors[1])} onClick={handleAboutOpen}>
              <Typography variant="body">How to Play</Typography>
            </Button>
            <Button sx={() => buttonStyle(colors[2])} onClick={handlePlayerNum}>
              {twoPlayer && <Typography variant="body">Play Phil</Typography>}
              {!twoPlayer && <Typography variant="body">2 player</Typography>}
            </Button>
            <Button sx={() => buttonStyle(colors[3])} onClick={newGame}>
              <Typography variant="body">New Game</Typography>
            </Button>
          </Box>
          <Box
            sx={{
              backgroundColor: "primary.main",
              display: "block",
              paddingTop: 4,
            }}
          >
            <Box
              sx={{
                margin: "auto",
                display: "block",
                maxWidth: "32rem",
              }}
            >
              <Paper
                elevation={0}
                square
                sx={{
                  width: 100,
                  height: 64,
                  display: "inline-block",
                  backgroundColor: colors[board[71] - 1],
                }}
              >
                <Typography
                  variant="subHeader"
                  sx={{
                    margin: 2,
                    textAlign: "center",
                    display: "block",
                  }}
                  color={textScore(board[71] - 1)}
                >
                  {scores[0]}
                </Typography>
              </Paper>
              {turn && !won[0] && (
                <FiberManualRecordIcon
                  color="secondary"
                  sx={{ paddingLeft: 1 }}
                />
              )}
              {won[0] && won[1] === 1 && (
                <EmojiEventsSharpIcon
                  color="secondary"
                  sx={{ paddingLeft: 1 }}
                />
              )}
              {won[0] && won[1] === 2 && (
                <CloseSharpIcon color="secondary" sx={{ paddingLeft: 1 }} />
              )}
              {won[0] && won[1] === 3 && (
                <AllInclusiveSharpIcon
                  color="secondary"
                  sx={{ paddingLeft: 1 }}
                />
              )}
              <Paper
                elevation={0}
                square
                sx={{
                  width: 100,
                  height: 64,
                  display: "inline-block",
                  backgroundColor: colors[board[18] - 1],
                  float: "right",
                }}
              >
                <Typography
                  variant="subHeader"
                  sx={{
                    margin: 2,
                    textAlign: "center",
                    display: "block",
                  }}
                  color={textScore(board[18] - 1)}
                >
                  {scores[2]}
                </Typography>
              </Paper>
              {!turn && !won[0] && (
                <FiberManualRecordIcon
                  color="secondary"
                  sx={{
                    paddingRight: 1,
                    paddingTop: 2.66,
                    float: "right",
                    display: "block",
                  }}
                />
              )}
              {won[0] && won[1] === 1 && (
                <CloseSharpIcon
                  color="secondary"
                  sx={{
                    paddingRight: 1,
                    paddingTop: 2.66,
                    float: "right",
                    display: "block",
                  }}
                />
              )}
              {won[0] && won[1] === 2 && (
                <EmojiEventsSharpIcon
                  color="secondary"
                  sx={{
                    paddingRight: 1,
                    paddingTop: 2.66,
                    float: "right",
                    display: "block",
                  }}
                />
              )}
              {won[0] && won[1] === 3 && (
                <AllInclusiveSharpIcon
                  color="secondary"
                  sx={{
                    paddingRight: 1,
                    paddingTop: 2.66,
                    float: "right",
                    display: "block",
                  }}
                />
              )}
              {twoPlayer && (
                <ColorPicker
                  blockedColors={blockedColors}
                  move={move}
                  turn={!turn}
                  active={!won[0]}
                />
              )}
              {!twoPlayer && (
                <Box
                  sx={{
                    display: "block",
                    paddingTop: "1rem",
                    paddingBottom: "2rem",
                    height: "5rem",
                    margin: "auto",
                    textAlign: "center",
                  }}
                >
                  <Typography
                    variant="subHeader"
                    color="#ffffff"
                    sx={{ marginBottom: 20 }}
                  >
                    {philFace}
                  </Typography>
                  <br />
                  <br />
                  <Typography
                    variant="subHeader"
                    color="#ffffff"
                    sx={{
                      display: "inline-block",
                      animation:
                        "animated-text 2s steps(30,end) 1s 1 normal both",
                    }}
                    id="phil"
                  >
                    <Typography id="writer"></Typography>
                    <Typewriter
                      onInit={(typewriter) => {
                        setTopTypewriter(typewriter);
                      }}
                    ></Typewriter>
                  </Typography>
                </Box>
              )}
              <Grid container spacing={0}>
                {board.map((num, key) => {
                  if (num !== 0) {
                    return (
                      <Paper
                        elevation={0}
                        square
                        sx={boxStyle(colors[num - 1])}
                        key={key}
                      />
                    );
                  }
                })}
              </Grid>
            </Box>
          </Box>
          <ColorPicker
            blockedColors={blockedColors}
            move={move}
            turn={turn}
            active={!won[0]}
          />
        </Box>
      </Box>
    </ThemeProvider>
  );
}

export default App;
