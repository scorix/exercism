export class ResistorColor {
  private colors: string[];
  private color_values: Map<string, number>;

  constructor(colors: string[]) {
    if (colors.length <= 1) {
      throw new Error("At least two colors need to be present");
    }

    this.colors = colors;
    this.color_values = new Map;
    this.color_values.set("black", 0);
    this.color_values.set("brown", 1);
    this.color_values.set("red", 2);
    this.color_values.set("orange", 3);
    this.color_values.set("yellow", 4);
    this.color_values.set("green", 5);
    this.color_values.set("blue", 6);
    this.color_values.set("violet", 7);
    this.color_values.set("grey", 8);
    this.color_values.set("white", 9);
  }

  value = (): number => {
    let n1 = this.color_values.get(this.colors[0]);
    let n2 = this.color_values.get(this.colors[1]);

    if (n1 == undefined || n2 == undefined) {
      throw new Error("All colors should be defined");
    }

    return n1*10 + n2;
  };
}
