export const defaultDoughnutOptions = {
  plugins: {
    legend: {
      display: false,
    },
  },
  borderColor: "#586e75",
  borderWidth: 0,
  cutout: "30%",
};

export const chartColors: string[] = [
  "#dc322f",
  "#268bd2",
  "#b58900",
  "#cb4b16",
  "#d33682",
  "#6c71c4",
  "#2aa198",
  "#859900",
];

export function formatFileSize(bytes: number, decimalPoint: number): string {
  if (bytes == 0) return "0 Bytes";
  var k = 1000,
    dm = decimalPoint || 2,
    sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
    i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
}
