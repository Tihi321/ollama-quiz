import { styled } from "solid-styled-components";
import { IconButton, MenuItem, Select, Box } from "@suid/material";
import { Component, createEffect, createSignal, onMount } from "solid-js";
import { Speaker } from "../icons/Speaker";
import { getStringValue, saveStringValue } from "../../hooks/local";

const Container = styled("div")`
  position: relative;
  background-color: ${(props) => props?.theme?.colors.darkBackground};
  color: ${(props) => props?.theme?.colors.text};
  padding: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
`;

const Options = styled(Box)`
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;

  .MuiInputBase-root {
    font-size: 14px;
    background-color: ${(props) => props?.theme?.colors.lightBackground};
  }

  .MuiSelect-select {
    padding: 8px 28px 8px 8px;
  }
`;

const IconButtonStyled = styled(IconButton)`
  svg {
    fill: ${(props) => props?.theme?.colors.text};
  }
`;

const TitleText = styled("h3")`
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  color: ${(props) => props?.theme?.colors.text};
  font-size: 18px;
  line-height: 1;
  margin: 0;
  flex: 1;

  @media (min-width: 700px) {
    font-size: 24px;
  }
`;

const SelectContainer = styled("div")`
  .MuiOutlinedInput-root {
    font-size: 12px;
  }
`;

interface HeaderProps {
  langugage: boolean;
}

export const Header: Component<HeaderProps> = (props) => {
  const [language, setLanguage] = createSignal("");
  let audioElement: any;

  createEffect(() => {
    if (audioElement) {
      audioElement.volume = 0.01;
      audioElement.loop = true;
    }
  });

  onMount(() => {
    const quizLanguage = getStringValue("kvizolamma/language");
    setLanguage(quizLanguage || "english");
  });

  return (
    <Container>
      <TitleText>Kvizollama</TitleText>
      <Options sx={{ justifyContent: props.langugage ? "space-between" : "flex-end" }}>
        {props.langugage && (
          <SelectContainer>
            <Select
              value={language()}
              onChange={(e) => {
                saveStringValue("language", e.target.value);
                setLanguage(e.target.value);
                window.location.reload();
              }}
            >
              <MenuItem value="english">Eng</MenuItem>
              <MenuItem value="croatian">Hr</MenuItem>
            </Select>
          </SelectContainer>
        )}

        <IconButtonStyled
          aria-label="toggle volume"
          onClick={() => {
            if (audioElement.paused) {
              audioElement.play();
            } else {
              audioElement.pause();
            }
          }}
        >
          <Speaker />
        </IconButtonStyled>
      </Options>
      <audio ref={audioElement}>
        <source src="https://start.tihomir-selak.from.hr/audio/just-relax.mp3" type="audio/mpeg" />
      </audio>
    </Container>
  );
};
