export class SourceMapDecoder {
  constructor(filename: string)
  parse(line: number, column: number): [string, number]
}
