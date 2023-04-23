import { Box, Paper } from '@mui/material';
import { useState } from 'react';

const ColorPicker = ({ blockedColors, move, turn, active } ) => {
    const colors = [
      "#e45968",
      "#fae253",
      "#b4d975",
      "#67b0f0",
      "#6b53a0",
      "#464747",
    ];

    const boxStyle = (color, size) => {
      return {
        width: size,
        height: size,
        display: "inline-block",
        backgroundColor: color,
        margin:'auto'
      };
    }

    const handleColorClick = (color) => {
      if (!blockedColors.includes(color) && turn && active) {
        move(color)
      }
    }

    return (
      <Box
      sx={{
        display:'flex'
      }}
      >
      <Box
        sx={{
          display: "flex",
          margin: 'auto',
          width: "30rem",
          paddingTop: "35px",
          paddingBottom: "35px",
          height:80
        }}
      >
        {colors.map((color, key) => {
            var size = 80;
            if (blockedColors.includes(colors.indexOf(color))) {
                size = 30;
            }
            return (
              <Box
                sx={{
                  width: 80,
                  height: 80,
                  display: 'inline-flex'
                }}
                key={key}
              >
                <Paper elevation={0} square sx={boxStyle(color, size)} onClick={() => handleColorClick(key)} key={key} />
              </Box>
            );
        })}
      </Box>
      </Box>
    );
}
 
export default ColorPicker;