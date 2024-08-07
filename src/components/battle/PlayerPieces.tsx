import { styled } from "solid-styled-components";
import { getPlayerColor } from "./utils";

const Soldier = styled.svg<{ player: number }>`
  position: absolute;
  width: 80%;
  height: 80%;
  top: 10%;
  left: 10%;
`;

const Flag = styled.svg<{ player: number }>`
  position: absolute;
  width: 60%;
  height: 60%;
  top: 20%;
  left: 20%;
`;

export const RenderSoldier = ({ player }: { player: number }) => (
  <Soldier viewBox="0 0 100 100" player={player}>
    <circle cx="50" cy="60" r="30" fill={getPlayerColor(player)} stroke="white" stroke-width="5" />
    <circle cx="50" cy="30" r="20" fill={getPlayerColor(player)} stroke="white" stroke-width="5" />
  </Soldier>
);

export const RenderFlag = ({ player }: { player: number }) => (
  <Flag viewBox="0 0 100 100" player={player}>
    <line x1="20" y1="20" x2="20" y2="80" stroke="black" stroke-width="5" />
    <path
      d="M20,20 L80,35 L20,50 Z"
      fill={getPlayerColor(player)}
      stroke="white"
      stroke-width="2"
    />
  </Flag>
);
