export function getPreviousVideoIndex(currentIndex: number): number {
  return Math.max(currentIndex - 1, 0);
}

export function getNextVideoIndex(
  currentIndex: number,
  totalVideos: number,
): number {
  if (totalVideos <= 0) {
    return 0;
  }

  return Math.min(currentIndex + 1, totalVideos - 1);
}

