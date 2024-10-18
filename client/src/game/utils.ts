export function createDisplayName(name: string): string {
  return name || "Anonymous";
}

export function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();

    img.onload = () => resolve(img);
    img.onerror = (err) => reject(err);
    img.src = src;
  });
}