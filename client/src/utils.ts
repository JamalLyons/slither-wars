export function createDisplayName(name: string): string {
    if(!name || name === "name") {
        return "Anonymous";
    } else {
        return name;
    }
}